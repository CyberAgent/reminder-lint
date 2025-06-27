use crate::args::ListCommand;
use anyhow::Error;
use reminder_lint_core::config::builder::ConfigBuilder;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct RemindList {
    pub expired: Vec<reminder_lint_core::remind::Remind>,
    pub upcoming: Vec<reminder_lint_core::remind::Remind>,
}

pub fn execute_list(command: ListCommand) -> Result<(), Error> {
    let conf = ConfigBuilder::new()
        .config_file_path(command.config_file_path)
        .ignore_file_path(command.ignore_file_path)
        .build()?;

    let mut expired = Vec::new();
    let mut upcoming = Vec::new();

    for remind in reminder_lint_core::reminders(&conf)?.reminds {
        if remind.datetime < chrono::Local::now().timestamp() {
            expired.push(remind);
        } else {
            upcoming.push(remind);
        }
    }

    let reminders = RemindList { expired, upcoming };

    if command.json {
        println!("{}", serde_json::to_string(&reminders)?);
        return Ok(());
    }

    for remind in &reminders.expired {
        println!(
            "{}:{} {}",
            remind.position.file, remind.position.line, remind.message
        );
    }

    for remind in &reminders.upcoming {
        println!(
            "{}:{} {}",
            remind.position.file, remind.position.line, remind.message
        );
    }

    Ok(())
}
