mod remind;
mod config;

use std::error::Error;
use crate::config::load_config;
use crate::remind::list_reminders;

const REMIND_IGNORE_CONFIG_FILE: &str = ".remindignore";
const REMIND_CONFIG_FILE: &str = "remind.yaml";

fn main() -> Result<(), Box<dyn Error>> {
    let conf = load_config(REMIND_CONFIG_FILE)?;
    let reminders = list_reminders(
        &conf.comment_regex,
        &conf.datetime_format,
        REMIND_IGNORE_CONFIG_FILE,
        &conf.search_directory)?;
    for remind in reminders {
        if remind.datetime < chrono::Local::now().timestamp() {
            println!("{}:{} {}", remind.position.file, remind.position.line, remind.message);
        }
    }
    Ok(())
}
