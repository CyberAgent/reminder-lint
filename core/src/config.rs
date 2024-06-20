use config::{Config as ConfigBuilder, ConfigError};
use serde::{Deserialize, Serialize};

const REMIND_ENV_PREFIX: &str = "REMIND";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub comment_regex: String,
    pub datetime_format: String,
    pub search_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            comment_regex: String::from(r"remind:\W?"),
            datetime_format: "%Y/%m/%d".to_string(),
            search_directory: ".".to_string(),
        }
    }
}

pub fn load_config(filename: &str) -> Result<Config, ConfigError> {
    let default = Config::default();
    let settings = ConfigBuilder::builder()
        .set_default("comment_regex", default.comment_regex)?
        .set_default("datetime_format", default.datetime_format)?
        .set_default("search_directory", default.search_directory)?
        .add_source(config::File::with_name(filename).required(false))
        .add_source(config::Environment::with_prefix(REMIND_ENV_PREFIX))
        .build()?;
    let conf = settings.try_deserialize::<Config>()?;
    Ok(conf)
}
