use reminder_lint_core::ReminderOptions;

use crate::args::RunCommand;

pub fn execute_run(command: RunCommand) {
    let options = ReminderOptions {
        config_file_path: command.config_file_path.as_deref(),
        ignore_file_path: command.ignore_file_path.as_deref(),
    };

    let reminders = match reminder_lint_core::reminders(&options) {
        Ok(reminders) => reminders,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    for remind in reminders.expired {
        println!(
            "{}:{} {}",
            remind.position.file, remind.position.line, remind.message
        );
    }
}
