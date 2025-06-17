use crate::args::InitCommand;
use crate::print::{pretty_print, Status};
use anyhow::Error;
use promptuity::prompts::{Confirm, Input, Select, SelectOption};
use promptuity::themes::FancyTheme;
use promptuity::{Promptuity, Term};
use reminder_lint_core::config::builder::{
    FileConfig, ValidateItem, DEFAULT_CONFIG_FILE_PATH, DEFAULT_IGNORE_FILE_PATH,
};

struct InitPromptResult {
    config: Option<FileConfig>,
    ignore: bool,
}

fn init_prompt() -> Result<InitPromptResult, Error> {
    // TODO: Add support for finding root of the project
    let is_config_file_exists = std::path::Path::new(DEFAULT_CONFIG_FILE_PATH).exists();
    let is_ignore_file_exists = std::path::Path::new(DEFAULT_IGNORE_FILE_PATH).exists();

    if is_config_file_exists && is_ignore_file_exists {
        return Err(Error::msg(format!(
            "{} and {} already exists",
            DEFAULT_CONFIG_FILE_PATH, DEFAULT_IGNORE_FILE_PATH
        )));
    }

    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.with_intro("Initialize reminder-lint").begin()?;

    let mut result = InitPromptResult {
        config: None,
        ignore: false,
    };

    let default_config = FileConfig::default();
    let hint_format = |hint: &str| format!("Default: {}", hint);

    if !is_config_file_exists {
        let comment_regex = p.prompt(
            Input::new("Please enter the comment regex")
                .with_required(false)
                .with_default(&default_config.comment_regex)
                .with_hint(hint_format(&default_config.comment_regex)),
        )?;

        let datetime_format = p
            .prompt(
                Select::new(
                    "Please select the datetime format",
                    vec![
                        SelectOption::new("%Y/%m/%d", "%Y/%m/%d"),
                        SelectOption::new("%Y/%m/%d %H:%M:%S", "%Y/%m/%d %H:%M:%S"),
                    ],
                )
                .as_mut(),
            )?
            .to_string();

        let search_directory = p.prompt(
            Input::new("Please enter the search directory")
                .with_required(false)
                .with_default(&default_config.search_directory)
                .with_hint(hint_format(&default_config.search_directory)),
        )?;

        let remind_if_no_date = p.prompt(
            Confirm::new("Remind if no datetime in comment?")
                .with_default(default_config.remind_if_no_date),
        )?;

        let mut validates = default_config.validates;
        validates.insert(
            "datetime".to_string(),
            ValidateItem {
                format: datetime_format.clone(),
            },
        );

        result.config = Some(FileConfig {
            comment_regex,
            datetime_format,
            search_directory,
            remind_if_no_date,
            validates,
        });
    }

    if !is_ignore_file_exists {
        let ignore = p.prompt(
            Confirm::new(format!("Create {} ?", DEFAULT_IGNORE_FILE_PATH)).with_default(true),
        )?;

        result.ignore = ignore;
    }

    p.finish()?;

    Ok(result)
}

pub fn execute_init(_command: InitCommand) -> Result<(), Error> {
    let InitPromptResult { config, ignore } = init_prompt()?;

    if let Some(config) = config {
        let yaml = serde_yaml::to_string(&config)?;
        std::fs::write(DEFAULT_CONFIG_FILE_PATH, yaml)?;

        pretty_print(
            format!("Successfully create ./{}", DEFAULT_CONFIG_FILE_PATH),
            Status::Success,
        );
    }

    if ignore {
        std::fs::write(
            DEFAULT_IGNORE_FILE_PATH,
            format!("{}\n", DEFAULT_CONFIG_FILE_PATH),
        )?;
        pretty_print(
            format!("Successfully create ./{}", DEFAULT_IGNORE_FILE_PATH),
            Status::Success,
        );
    }

    Ok(())
}
