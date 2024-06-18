pub const DEFAULT_CONFIG_FILE_PATH: &str = "remind.yaml";
pub const DEFAULT_IGNORE_FILE_PATH: &str = ".remindignore";

pub struct ReminderOptions<'a> {
    pub config_file_path: Option<&'a str>,
    pub ignore_file_path: Option<&'a str>,
}

impl ReminderOptions<'_> {
    pub fn config_file(&self) -> &str {
        self.config_file_path.unwrap_or(DEFAULT_CONFIG_FILE_PATH)
    }

    pub fn ignore_file(&self) -> &str {
        self.ignore_file_path.unwrap_or(DEFAULT_IGNORE_FILE_PATH)
    }
}
