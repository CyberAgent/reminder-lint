use std::collections::HashMap;

use builder::ValidateItem;
use serde::{Deserialize, Serialize};

pub mod builder;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    comment_regex: String,
    datetime_format: String,
    search_directory: String,
    ignore_file_path: String,
    sort_by_deadline: bool,
    remind_if_no_date: bool,
    validates: HashMap<String, ValidateItem>,
}

impl Config {
    pub fn comment_regex(&self) -> &str {
        &self.comment_regex
    }

    pub fn datetime_format(&self) -> &str {
        &self.datetime_format
    }

    pub fn search_directory(&self) -> &str {
        &self.search_directory
    }

    pub fn ignore_file_path(&self) -> &str {
        &self.ignore_file_path
    }

    pub fn sort_by_deadline(&self) -> bool {
        self.sort_by_deadline
    }

    pub fn remind_if_no_date(&self) -> bool {
        self.remind_if_no_date
    }

    pub fn validates(&self) -> &HashMap<String, ValidateItem> {
        &self.validates
    }
}
