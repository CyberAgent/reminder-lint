use config::{Config as ConfigBuilder, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub comment_regex: String,
    pub datetime_format: String,
    pub search_directory: String,
}

const DEFAULT_REMIND_COMMENT_REGEX: &str = r"remind:\W?";
const DEFAULT_REMIND_DATETIME_FORMAT: &str = "%Y/%m/%d";
const REMIND_ENV_PREFIX: &str = "REMIND";

pub fn load_config(filename: &str) -> Result<Config, ConfigError> {
    let settings = ConfigBuilder::builder()
        .set_default("comment_regex", DEFAULT_REMIND_COMMENT_REGEX)?
        .set_default("datetime_format", DEFAULT_REMIND_DATETIME_FORMAT)?
        .set_default("search_directory", ".")?
        .add_source(config::File::with_name(filename).required(false))
        .add_source(config::Environment::with_prefix(REMIND_ENV_PREFIX))
        .build()?;
    let conf = settings.try_deserialize::<Config>()?;
    Ok(conf)
}
