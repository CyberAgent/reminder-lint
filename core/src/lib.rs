pub mod config;
pub mod error;
pub mod remind;

use config::Config;
use error::ReminderLintError;
use remind::list_reminders;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Reminders {
    pub reminds: Vec<remind::Remind>,
}

pub fn reminders(conf: &Config) -> Result<Reminders, ReminderLintError> {
    let reminders = list_reminders(conf).map_err(ReminderLintError::from)?;

    Ok(Reminders { reminds: reminders })
}
