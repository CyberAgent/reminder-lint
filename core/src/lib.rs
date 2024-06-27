pub mod config;
pub mod error;
pub mod remind;

use config::Config;
use error::ReminderLintError;
use remind::list_reminders;

#[derive(Debug)]
pub struct Reminders {
    pub expired: Vec<remind::Remind>,
    pub upcoming: Vec<remind::Remind>,
}

pub fn reminders(conf: &Config) -> Result<Reminders, ReminderLintError> {
    let reminders = list_reminders(&conf).map_err(|e| ReminderLintError::from(e))?;

    let mut expired = Vec::new();
    let mut upcoming = Vec::new();

    for remind in reminders {
        if remind.datetime < chrono::Local::now().timestamp() {
            expired.push(remind);
        } else {
            upcoming.push(remind);
        }
    }

    let reminders = Reminders { expired, upcoming };

    Ok(reminders)
}
