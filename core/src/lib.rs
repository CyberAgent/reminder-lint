mod config;
mod error;
mod options;
mod remind;

use error::ReminderLintError;
pub use options::ReminderOptions;

use crate::config::load_config;
use crate::remind::list_reminders;

#[derive(Debug)]
pub struct Reminders {
    pub expired: Vec<remind::Remind>,
    pub upcoming: Vec<remind::Remind>,
}

pub fn reminders(options: &ReminderOptions) -> Result<Reminders, ReminderLintError> {
    let conf = load_config(options.config_file()).map_err(|e| ReminderLintError::from(e))?;

    let reminders = list_reminders(
        &conf.comment_regex,
        &conf.datetime_format,
        options.ignore_file(),
        &conf.search_directory,
    )
    .map_err(|e| ReminderLintError::from(e))?;

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
