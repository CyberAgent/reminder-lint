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
pub struct FileConfig {
    pub comment_regex: String,
    pub datetime_format: String,
    pub search_directory: String,
    pub remind_if_no_date: bool,
    pub validates: HashMap<String, ValidateItem>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            comment_regex: String::from(r"remind:\W?"),
            datetime_format: "%Y/%m/%d".to_string(),
            search_directory: ".".to_string(),
            remind_if_no_date: false,
            validates: HashMap::new(),
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
            "validates",
            default
                .validates
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

        if !file_config.validates.is_empty() && contains_meta_matcher(&file_config.comment_regex) {
            return Err(ConfigError::Message(
                "Validation and meta matcher features cannot be used together".to_string(),
            ));
        }

        Ok(Config {
            comment_regex: file_config.comment_regex,
            datetime_format: file_config.datetime_format,
            search_directory: file_config.search_directory,
            remind_if_no_date,
            validates: file_config.validates,
            ignore_file_path,
            sort_by_deadline: self.sort_by_deadline.unwrap_or(false),
        })
    }
}
