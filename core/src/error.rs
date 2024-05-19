use anyhow::Error as AnyHowError;
use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReminderLintError {
    #[error("Failed to load config: {0}")]
    FailedLoadConfig(ConfigError),

    #[error("Failed to get reminders: {0}")]
    FailedGetReminders(AnyHowError),
}

impl From<ConfigError> for ReminderLintError {
    fn from(err: ConfigError) -> Self {
        ReminderLintError::FailedLoadConfig(err)
    }
}

impl From<AnyHowError> for ReminderLintError {
    fn from(err: AnyHowError) -> Self {
        ReminderLintError::FailedGetReminders(err)
    }
}
