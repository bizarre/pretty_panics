use proc_macro::{self, TokenStream};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, ExprPath, ItemFn, PathArguments, ReturnType, Type};

#[proc_macro_attribute]
#[proc_macro_error]
#[doc(hidden)]
pub fn pretty_panic(attrs: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the function
    let input_fn = parse_macro_input!(input as ItemFn);

    // Check if the function is `main`
    if input_fn.sig.ident != "main" {
        abort_call_site!("The `#[pretty]` attribute can only be used on the `main` function.");
    }

    let mut format_error: Option<ExprPath> = None;
    let mut format_panic: Option<ExprPath> = None;
    // Parse the attributes using syn
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("formatter") {
            format_error = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("panic_formatter") {
            format_panic = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported pretty_panic property"))
        }
    });

    parse_macro_input!(attrs with parser);

    let format_error = if let Some(format_error) = &format_error {
        quote! { #format_error }
    } else {
        if cfg!(feature = "default_formatters") {
            quote! { pretty_panic::default_formatters::error_formatter }
        } else {
            abort_call_site!(
                "`formatter` not provided and `default_formatters` feature is not enabled."
            );
        }
    };

    let format_panic = if let Some(format_panic) = &format_panic {
        quote! { eprintln!("{}", #format_panic(panic_hook_info, message.to_string())); }
    } else {
        if cfg!(feature = "default_formatters") {
            quote! { eprintln!("{}", pretty_panic::default_formatters::panic_formatter(panic_hook_info, message.to_string())); }
        } else {
            abort_call_site!(
                "`panic_formatter` not provided and `default_formatters` feature is not enabled."
            );
        }
    };

    let output = match &input_fn.sig.output {
        ReturnType::Type(_, ty) => {
            // Check if the return type is `Result<T, E>`
            if let Type::Path(type_path) = &**ty {
                let path = &type_path.path;

                // Check if the path is `Result`
                if let Some(segment) = path.segments.last() {
                    if segment.ident == "Result" {
                        // Ensure that the generic arguments are exactly two
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if args.args.len() >= 1 {
                                let return_type = quote! { #ty };
                                let body = &input_fn.block;
                                // Generate a new function with the original content
                                let new_fn_name = syn::Ident::new(
                                    &format!("{}_wrapped", input_fn.sig.ident),
                                    input_fn.sig.ident.span(),
                                );

                                let gen_new_fn = quote! {
                                    fn #new_fn_name() -> #return_type {
                                        // Original function body
                                        #body
                                    }
                                };

                                // Generate the modified main function that calls the new function
                                let gen_main = quote! {
                                    fn main() {
                                        std::panic::set_hook(Box::new(|panic_hook_info| {
                                            let payload = panic_hook_info.payload();
                                            if let Some(message) = payload.downcast_ref::<String>() {
                                                #format_panic
                                            } else if let Some(message) = payload.downcast_ref::<&str>() {
                                                #format_panic
                                            }
                                        }));

                                        if let Err(e) = #new_fn_name() {
                                            eprintln!("{}", #format_error(&e));
                                            std::process::exit(1);
                                        }
                                    }
                                };

                                // Combine the two function definitions
                                let expanded = quote! {
                                    #gen_new_fn
                                    #gen_main
                                };

                                return expanded.into();
                            }
                        }
                        abort_call_site!("The `main` function must return a `Result<T, E>` type.");
                    }
                }
            }

            quote! {
                #input_fn
            }
            .into()
        }
        _ => quote! {
            #input_fn
        }
        .into(),
    };

    output
}
