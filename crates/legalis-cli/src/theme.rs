//! Terminal theming and output formatting utilities.

use crate::ColorTheme;
use colored::{Color, Colorize};

/// Theme configuration for colored output.
#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub dimmed: Color,
    pub highlight: Color,
}

impl Theme {
    /// Create a theme from a ColorTheme enum.
    pub fn from_color_theme(theme: &ColorTheme) -> Self {
        match theme {
            ColorTheme::Default => Self::default_theme(),
            ColorTheme::Dark => Self::dark_theme(),
            ColorTheme::Light => Self::light_theme(),
            ColorTheme::Monokai => Self::monokai_theme(),
            ColorTheme::Solarized => Self::solarized_theme(),
            ColorTheme::None => Self::no_color_theme(),
        }
    }

    fn default_theme() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            dimmed: Color::BrightBlack,
            highlight: Color::Magenta,
        }
    }

    fn dark_theme() -> Self {
        Self {
            primary: Color::BrightCyan,
            secondary: Color::BrightBlue,
            success: Color::BrightGreen,
            warning: Color::BrightYellow,
            error: Color::BrightRed,
            info: Color::BrightCyan,
            dimmed: Color::BrightBlack,
            highlight: Color::BrightMagenta,
        }
    }

    fn light_theme() -> Self {
        Self {
            primary: Color::Blue,
            secondary: Color::Cyan,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Blue,
            dimmed: Color::Black,
            highlight: Color::Magenta,
        }
    }

    fn monokai_theme() -> Self {
        Self {
            primary: Color::TrueColor {
                r: 102,
                g: 217,
                b: 239,
            }, // Cyan
            secondary: Color::TrueColor {
                r: 166,
                g: 226,
                b: 46,
            }, // Green
            success: Color::TrueColor {
                r: 166,
                g: 226,
                b: 46,
            }, // Green
            warning: Color::TrueColor {
                r: 253,
                g: 151,
                b: 31,
            }, // Orange
            error: Color::TrueColor {
                r: 249,
                g: 38,
                b: 114,
            }, // Pink
            info: Color::TrueColor {
                r: 102,
                g: 217,
                b: 239,
            }, // Cyan
            dimmed: Color::TrueColor {
                r: 117,
                g: 113,
                b: 94,
            }, // Gray
            highlight: Color::TrueColor {
                r: 174,
                g: 129,
                b: 255,
            }, // Purple
        }
    }

    fn solarized_theme() -> Self {
        Self {
            primary: Color::TrueColor {
                r: 38,
                g: 139,
                b: 210,
            }, // Blue
            secondary: Color::TrueColor {
                r: 42,
                g: 161,
                b: 152,
            }, // Cyan
            success: Color::TrueColor {
                r: 133,
                g: 153,
                b: 0,
            }, // Green
            warning: Color::TrueColor {
                r: 181,
                g: 137,
                b: 0,
            }, // Yellow
            error: Color::TrueColor {
                r: 220,
                g: 50,
                b: 47,
            }, // Red
            info: Color::TrueColor {
                r: 42,
                g: 161,
                b: 152,
            }, // Cyan
            dimmed: Color::TrueColor {
                r: 88,
                g: 110,
                b: 117,
            }, // Base01
            highlight: Color::TrueColor {
                r: 211,
                g: 54,
                b: 130,
            }, // Magenta
        }
    }

    fn no_color_theme() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::White,
            success: Color::White,
            warning: Color::White,
            error: Color::White,
            info: Color::White,
            dimmed: Color::White,
            highlight: Color::White,
        }
    }

    /// Apply primary color to text.
    pub fn primary(&self, text: &str) -> colored::ColoredString {
        text.color(self.primary)
    }

    /// Apply success color to text.
    pub fn success(&self, text: &str) -> colored::ColoredString {
        text.color(self.success)
    }

    /// Apply warning color to text.
    pub fn warning(&self, text: &str) -> colored::ColoredString {
        text.color(self.warning)
    }

    /// Apply error color to text.
    pub fn error(&self, text: &str) -> colored::ColoredString {
        text.color(self.error)
    }

    /// Apply info color to text.
    pub fn info(&self, text: &str) -> colored::ColoredString {
        text.color(self.info)
    }

    /// Apply dimmed color to text.
    pub fn dimmed(&self, text: &str) -> colored::ColoredString {
        text.color(self.dimmed)
    }

    /// Apply highlight color to text.
    pub fn highlight(&self, text: &str) -> colored::ColoredString {
        text.color(self.highlight)
    }
}

/// Output context for formatting decisions.
pub struct OutputContext {
    pub theme: Theme,
    pub use_emoji: bool,
    pub width: usize,
    pub use_pager: bool,
}

impl OutputContext {
    /// Create a new output context.
    pub fn new(theme: Theme, use_emoji: bool, width: Option<usize>, use_pager: bool) -> Self {
        let terminal_width = width.unwrap_or_else(|| {
            terminal_size::terminal_size()
                .map(|(w, _)| w.0 as usize)
                .unwrap_or(80)
        });

        Self {
            theme,
            use_emoji,
            width: terminal_width,
            use_pager,
        }
    }

    /// Get a success symbol (emoji or text).
    pub fn success_symbol(&self) -> &'static str {
        if self.use_emoji { "✓" } else { "[OK]" }
    }

    /// Get an error symbol (emoji or text).
    pub fn error_symbol(&self) -> &'static str {
        if self.use_emoji { "✗" } else { "[ERROR]" }
    }

    /// Get a warning symbol (emoji or text).
    pub fn warning_symbol(&self) -> &'static str {
        if self.use_emoji { "⚠" } else { "[WARN]" }
    }

    /// Get an info symbol (emoji or text).
    pub fn info_symbol(&self) -> &'static str {
        if self.use_emoji { "ℹ" } else { "[INFO]" }
    }

    /// Wrap text to fit terminal width.
    pub fn wrap_text(&self, text: &str) -> String {
        if text.len() <= self.width {
            return text.to_string();
        }

        let mut result = String::new();
        let mut current_line_len = 0;

        for word in text.split_whitespace() {
            if current_line_len + word.len() + 1 > self.width {
                result.push('\n');
                current_line_len = 0;
            }

            if current_line_len > 0 {
                result.push(' ');
                current_line_len += 1;
            }

            result.push_str(word);
            current_line_len += word.len();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::from_color_theme(&ColorTheme::Default);
        assert_eq!(theme.success, Color::Green);
        assert_eq!(theme.error, Color::Red);
    }

    #[test]
    fn test_output_context() {
        let theme = Theme::from_color_theme(&ColorTheme::Default);
        let ctx = OutputContext::new(theme, true, Some(80), false);
        assert_eq!(ctx.success_symbol(), "✓");
        assert_eq!(ctx.width, 80);
    }

    #[test]
    fn test_no_emoji_mode() {
        let theme = Theme::from_color_theme(&ColorTheme::Default);
        let ctx = OutputContext::new(theme, false, Some(80), false);
        assert_eq!(ctx.success_symbol(), "[OK]");
        assert_eq!(ctx.error_symbol(), "[ERROR]");
    }

    #[test]
    fn test_text_wrapping() {
        let theme = Theme::from_color_theme(&ColorTheme::Default);
        let ctx = OutputContext::new(theme, true, Some(20), false);
        let text = "This is a very long text that should be wrapped";
        let wrapped = ctx.wrap_text(text);
        assert!(wrapped.contains('\n'));
    }
}
