mod config;
mod remind;

use crate::config::load_config;
use crate::remind::list_reminders;
use std::error::Error;

const REMIND_IGNORE_CONFIG_FILE: &str = ".remindignore";
const REMIND_CONFIG_FILE: &str = "remind.yaml";

pub struct Reminders {
    pub expired: Vec<remind::Remind>,
    pub upcoming: Vec<remind::Remind>,
}

pub fn reminders() -> Result<Reminders, Box<dyn Error>> {
    let conf = load_config(REMIND_CONFIG_FILE)?;
    let reminders = list_reminders(
        &conf.comment_regex,
        &conf.datetime_format,
        REMIND_IGNORE_CONFIG_FILE,
        &conf.search_directory,
    )?;

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
