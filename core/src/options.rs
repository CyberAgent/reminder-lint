pub const DEFAULT_CONFIG_FILE_PATH: &str = "remind.yml";
pub const DEFAULT_IGNORE_FILE_PATH: &str = ".remindignore";

pub struct ReminderOptions<'a> {
    config_file_path: Option<&'a str>,
    ignore_file_path: Option<&'a str>,
}

impl<'a> ReminderOptions<'a> {
    // Public constructor to initialize the options with default values
    pub fn builder() -> ReminderOptionsBuilder<'a> {
        ReminderOptionsBuilder {
            config_file_path: None,
            ignore_file_path: None,
        }
    }

    // Getters for the file paths
    pub fn config_file(&self) -> &str {
        self.config_file_path.unwrap_or(DEFAULT_CONFIG_FILE_PATH)
    }

    pub fn ignore_file(&self) -> &str {
        self.ignore_file_path.unwrap_or(DEFAULT_IGNORE_FILE_PATH)
    }
}

pub struct ReminderOptionsBuilder<'a> {
    config_file_path: Option<&'a str>,
    ignore_file_path: Option<&'a str>,
}

impl<'a> ReminderOptionsBuilder<'a> {
    pub fn config_file_path(mut self, config_file_path: Option<&'a str>) -> Self {
        self.config_file_path = config_file_path;
        self
    }

    pub fn ignore_file_path(mut self, ignore_file_path: Option<&'a str>) -> Self {
        self.ignore_file_path = ignore_file_path;
        self
    }

    pub fn build(self) -> ReminderOptions<'a> {
        ReminderOptions {
            config_file_path: self.config_file_path,
            ignore_file_path: self.ignore_file_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reminder_options() {
        let options = ReminderOptions::builder()
            .config_file_path(Some("config.yml"))
            .ignore_file_path(Some("ignore.yml"))
            .build();

        assert_eq!(options.config_file(), "config.yml");
        assert_eq!(options.ignore_file(), "ignore.yml");
    }

    #[test]
    fn test_default_reminder_options() {
        let options = ReminderOptions::builder().build();

        assert_eq!(options.config_file(), DEFAULT_CONFIG_FILE_PATH);
        assert_eq!(options.ignore_file(), DEFAULT_IGNORE_FILE_PATH);
    }
}
