use anyhow::Error;
use reminder_lint_core::ReminderOptions;

use crate::args::RunCommand;

pub fn execute_run(command: RunCommand) -> Result<(), Error> {
    let options = ReminderOptions::builder()
        .config_file_path(command.config_file_path.as_deref())
        .ignore_file_path(command.ignore_file_path.as_deref())
        .build();

    let reminders = reminder_lint_core::reminders(&options)?;
    for remind in reminders.expired {
        println!(
            "{}:{} {}",
            remind.position.file, remind.position.line, remind.message
        );
    }

    Ok(())
}
