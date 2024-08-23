#![doc = include_str!("../README.md")]

pub use pretty_panic_proc_macro::pretty_panic;

#[cfg(feature = "default_formatters")]
#[doc(hidden)]
pub mod default_formatters {
    use anstyle::{Ansi256Color, Color, Reset, Style};
    use std::{error::Error, panic::PanicHookInfo};

    const BASE_STYLE: Style = Style::new().fg_color(Some(Color::Ansi256(Ansi256Color(210))));
    const HIGHLIGHT_STYLE: Style = Style::new().fg_color(Some(Color::Ansi256(Ansi256Color(210))));
    const TEXT_STYLE: Style = Style::new().fg_color(Some(Color::Ansi256(Ansi256Color(224))));
    const RESET_STYLE: Reset = Reset;

    pub fn error_formatter(error: &dyn Error) -> String {
        format!("{BASE_STYLE}Error: {RESET_STYLE}{HIGHLIGHT_STYLE}{error}")
    }

    pub fn panic_formatter(panic_hook_info: &PanicHookInfo, panic_message: String) -> String {
        let location = panic_hook_info
            .location()
            .map(|loc| loc.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let location_message = format!("{BASE_STYLE}Panicked at {HIGHLIGHT_STYLE}{location}");
        let message = format!("{RESET_STYLE}{TEXT_STYLE}{panic_message}");
        format!("{location_message}\n{message}")
    }
}
