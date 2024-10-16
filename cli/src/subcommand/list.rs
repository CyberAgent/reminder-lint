use crate::args::ListCommand;
use anyhow::Error;
use reminder_lint_core::config::builder::ConfigBuilder;

pub fn execute_list(command: ListCommand) -> Result<(), Error> {
    let conf = ConfigBuilder::new()
        .config_file_path(command.config_file_path)
        .ignore_file_path(command.ignore_file_path)
        .build()?;

    let reminders = reminder_lint_core::reminders(&conf)?;

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
