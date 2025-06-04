use anyhow::Error;
use reminder_lint_core::config::builder::ConfigBuilder;

use crate::args::RunCommand;

pub fn execute_run(command: RunCommand) -> Result<(), Error> {
    let conf = ConfigBuilder::new()
        .config_file_path(command.config_file_path)
        .ignore_file_path(command.ignore_file_path)
        .sort_by_deadline(command.sort_by_deadline)
        .build()?;

    let reminders = reminder_lint_core::reminders(&conf)?;
    let expired = reminders
        .reminds
        .iter()
        .filter(|remind| remind.datetime < chrono::Local::now().timestamp())
        .collect::<Vec<_>>();

    for remind in &expired {
        println!(
            "{}:{} {}",
            remind.position.file, remind.position.line, remind.message
        );
    }

    if !expired.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
