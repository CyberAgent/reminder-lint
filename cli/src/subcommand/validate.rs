use std::collections::HashMap;

use crate::{args::ValidateCommand, print::pretty_print};
use anyhow::Error;
use regex::Regex;
use reminder_lint_core::{
    config::builder::{ConfigBuilder, ValidateItem},
    remind::datetime_format_to_regex,
};

struct InvalidRemind {
    pub remind: reminder_lint_core::remind::Remind,
    pub unmatched: HashMap<String, ValidateItem>,
}

pub fn execute_validate(command: ValidateCommand) -> Result<(), Error> {
    let conf = ConfigBuilder::new()
        .config_file_path(command.config_file_path)
        .ignore_file_path(command.ignore_file_path)
        .sort_by_deadline(command.sort_by_deadline)
        .remind_if_no_date(Some(true)) // Always validate even if no date
        .build()?;

    let reminders = reminder_lint_core::reminders(&conf)?;
    let mut invalid_reminds = Vec::new();

    for remind in reminders.reminds {
        let mut unmatched = HashMap::new();
        for (name, validate) in conf.validate() {
            let reg_str = datetime_format_to_regex(&validate.format);
            let format_regex = Regex::new(&reg_str).unwrap();
            if !format_regex.is_match(&remind.message) {
                unmatched.insert(name.clone(), validate.clone());
            }
        }

        if !unmatched.is_empty() {
            invalid_reminds.push(InvalidRemind { remind, unmatched });
        }
    }

    if !invalid_reminds.is_empty() {
        pretty_print(
            format!("found {} invalid reminders:", invalid_reminds.len()),
            crate::print::Status::Error,
        );

        for invalid_remind in &invalid_reminds {
            let remind = &invalid_remind.remind;
            let unmatched = &invalid_remind.unmatched;

            print!(
                "{}:{} {}",
                remind.position.file, remind.position.line, remind.message
            );
            for (name, format) in unmatched {
                println!(
                    "Missing `{}` format: \x1b[31m{}\x1b[0m ",
                    name, format.format
                );
            }
            println!();
        }
        std::process::exit(1);
    }

    pretty_print("all reminders are valid", crate::print::Status::Success);
    Ok(())
}
