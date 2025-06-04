use crate::remind::meta::contains_meta_matcher;
use config::{Config as FileConfigBuilder, ConfigError, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Config;

pub const DEFAULT_CONFIG_FILE_PATH: &str = "remind.yml";
pub const DEFAULT_IGNORE_FILE_PATH: &str = ".remindignore";
const REMIND_ENV_PREFIX: &str = "REMIND";

pub struct ConfigBuilder {
    config_file_path: Option<String>,
    ignore_file_path: Option<String>,
    sort_by_deadline: Option<bool>,
    remind_if_no_date: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ValidateItem {
    pub format: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TriggerItem {
    #[serde(rename = "datetime")]
    pub datetime: String,
}

impl Default for TriggerItem {
    fn default() -> Self {
        TriggerItem {
            datetime: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileConfig {
    pub comment_regex: String,
    #[serde(skip_serializing)]
    pub datetime_format: String,
    pub search_directory: String,
    pub remind_if_no_date: bool,
    #[serde(default)]
    pub trigger: TriggerItem,
    pub validate: HashMap<String, ValidateItem>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            comment_regex: String::from(r"remind:\W?"),
            datetime_format: "".to_string(),
            search_directory: ".".to_string(),
            remind_if_no_date: false,
            trigger: TriggerItem {
                datetime: "datetime".to_string(),
            },
            validate: HashMap::new(),
        }
    }
}

impl From<ValidateItem> for Value {
    fn from(item: ValidateItem) -> Self {
        let mut map = HashMap::<String, Value>::new();
        map.insert("format".to_string(), item.format.into());
        Value::from(map)
    }
}

fn load_config(filename: &str) -> Result<FileConfig, ConfigError> {
    let default = FileConfig::default();
    let settings = FileConfigBuilder::builder()
        .set_default("comment_regex", default.comment_regex)?
        .set_default("datetime_format", default.datetime_format)?
        .set_default("search_directory", default.search_directory)?
        .set_default("remind_if_no_date", default.remind_if_no_date)?
        .set_default(
            "validate",
            default
                .validate
                .into_iter()
                .map(|(k, v)| (k, Value::from(v)))
                .collect::<HashMap<String, Value>>(),
        )?
        .add_source(config::File::with_name(filename).required(false))
        .add_source(config::Environment::with_prefix(REMIND_ENV_PREFIX))
        .build()?;
    let conf = settings.try_deserialize::<FileConfig>()?;
    Ok(conf)
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config_file_path: None,
            ignore_file_path: None,
            sort_by_deadline: None,
            remind_if_no_date: None,
        }
    }

    pub fn config_file_path(mut self, config_file_path: Option<String>) -> Self {
        self.config_file_path = config_file_path;
        self
    }

    pub fn ignore_file_path(mut self, ignore_file_path: Option<String>) -> Self {
        self.ignore_file_path = ignore_file_path;
        self
    }

    pub fn sort_by_deadline(mut self, sort_by_deadline: Option<bool>) -> Self {
        self.sort_by_deadline = sort_by_deadline;
        self
    }

    pub fn remind_if_no_date(mut self, remind_if_no_date: Option<bool>) -> Self {
        self.remind_if_no_date = remind_if_no_date;
        self
    }

    pub fn build(self) -> Result<Config, ConfigError> {
        let config_file_path = self
            .config_file_path
            .unwrap_or(DEFAULT_CONFIG_FILE_PATH.to_string());
        let ignore_file_path = self
            .ignore_file_path
            .unwrap_or(DEFAULT_IGNORE_FILE_PATH.to_string());
        let file_config = load_config(&config_file_path)?;
        let remind_if_no_date = self
            .remind_if_no_date
            .unwrap_or(file_config.remind_if_no_date);

        if !file_config.validate.is_empty() && contains_meta_matcher(&file_config.comment_regex) {
            return Err(ConfigError::Message(
                "Validation and meta matcher features cannot be used together".to_string(),
            ));
        }

        if file_config.datetime_format.is_empty() && file_config.trigger.datetime.is_empty() {
            return Err(ConfigError::Message(
                "trigger.datetime must be set".to_string(),
            ));
        }

        if !file_config.datetime_format.is_empty() {
            println!("\x1b[33m[DEPRECATED]\x1b[0m datetime_format is deprecated, please use trigger.datetime instead.");
        }

        Ok(Config {
            comment_regex: file_config.comment_regex,
            datetime_format: file_config.datetime_format,
            search_directory: file_config.search_directory,
            remind_if_no_date,
            validate: file_config.validate,
            ignore_file_path,
            trigger: file_config.trigger,
            sort_by_deadline: self.sort_by_deadline.unwrap_or(false),
        })
    }
}
