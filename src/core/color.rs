use colored::Colorize;

pub struct ColorScheme {
    pub enabled: bool,
}

impl ColorScheme {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn header(&self, text: &str) -> String {
        if self.enabled {
            text.blue().bold().to_string()
        } else {
            text.to_string()
        }
    }

    pub fn command(&self, text: &str) -> String {
        if self.enabled {
            text.green().to_string()
        } else {
            text.to_string()
        }
    }

    pub fn warning(&self, text: &str) -> String {
        if self.enabled {
            text.red().to_string()
        } else {
            text.to_string()
        }
    }

    pub fn metadata(&self, text: &str) -> String {
        if self.enabled {
            text.cyan().to_string()
        } else {
            text.to_string()
        }
    }

    pub fn badge(&self, text: &str) -> String {
        if self.enabled {
            text.yellow().dimmed().to_string()
        } else {
            text.to_string()
        }
    }

    pub fn dim(&self, text: &str) -> String {
        if self.enabled {
            text.dimmed().to_string()
        } else {
            text.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_disabled() {
        let colors = ColorScheme::new(false);
        assert_eq!(colors.header("Test"), "Test");
        assert_eq!(colors.command("Test"), "Test");
        assert_eq!(colors.warning("Test"), "Test");
    }

    #[test]
    fn test_color_enabled() {
        let colors = ColorScheme::new(true);
        assert!(colors.header("Test").contains("Test"));
        assert!(colors.command("Test").contains("Test"));
        assert!(colors.warning("Test").contains("Test"));
    }
}
