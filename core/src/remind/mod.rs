use ::regex::{Regex, RegexBuilder};
use anyhow::Error;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use grep_regex::RegexMatcherBuilder;
use grep_searcher::sinks::UTF8;
use grep_searcher::SearcherBuilder;
use ignore::WalkBuilder;
use meta::{convert_meta_regex, extract_placeholders};
use serde::Serialize;
use std::collections::HashMap;
use std::io;

use crate::config::Config;

pub(crate) mod meta;

#[derive(Debug, Serialize)]
pub struct Remind {
    pub datetime: i64,
    pub message: String,
    pub position: Position,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct Position {
    pub file: String,
    pub line: u64,
}

pub fn list_reminders(config: &Config) -> Result<Vec<Remind>, Error> {
    let meta_regex = convert_meta_regex(config.comment_regex());
    let matcher = RegexMatcherBuilder::new().build(&meta_regex)?;

    let mut searcher_builder = SearcherBuilder::new();
    let mut searcher = searcher_builder
        .binary_detection(grep_searcher::BinaryDetection::quit(b'\0'))
        .line_number(true)
        .build();

    let mut builder = WalkBuilder::new(config.search_directory());
    let walker = builder
        .hidden(false)
        .add_custom_ignore_filename(config.ignore_file_path())
        .ignore(true)
        .parents(false)
        .build();

    let datetime = config.datetime_format().to_owned();
    let datetime_regex = datetime_format_to_regex(&datetime);
    let datetime_regex = RegexBuilder::new(&datetime_regex).build()?;

    let mut reminds = walker
        .filter_map(|e| {
            let mut reminders: Vec<Remind> = vec![];
            let entry = e.ok()?;
            let path = line_processor(
                &mut reminders,
                entry.path().display().to_string(),
                datetime.clone(),
                &datetime_regex,
                config.comment_regex(),
            );

            let _result = searcher.search_path(&matcher, entry.path(), path);
            Some(reminders)
        })
        .flatten()
        .collect::<Vec<_>>();

    if !config.remind_if_no_date() {
        reminds.retain(|r| r.datetime != 0);
    }

    if config.sort_by_deadline() {
        reminds.sort_by(|a, b| a.datetime.cmp(&b.datetime));
    }

    Ok(reminds)
}

fn parse_datetime(v: &str, format: &str) -> Result<i64, Error> {
    match v {
        "" => Ok(0),
        _ => {
            let parsed_date =
                if format.contains("%H") || format.contains("%M") || format.contains("%S") {
                    NaiveDateTime::parse_from_str(v, format)?
                        .and_utc()
                        .timestamp()
                } else {
                    NaiveDate::parse_from_str(v, format)?
                        .and_time(NaiveTime::default())
                        .and_utc()
                        .timestamp()
                };
            Ok(parsed_date)
        }
    }
}

fn line_processor<'a>(
    reminds: &'a mut Vec<Remind>,
    entry_path: String,
    datetime_format: String,
    datetime_regex: &'a Regex,
    comment_regex: &'a str,
) -> UTF8<impl FnMut(u64, &str) -> Result<bool, io::Error> + 'a> {
    UTF8(move |line_num, line| {
        let datetime_str = datetime_regex.find(line).map_or("", |m| m.as_str());
        let parsed = parse_datetime(datetime_str, &datetime_format);
        let datetime = parsed.unwrap_or_else(|_| {
            eprintln!("Failed to parse datetime: {}", datetime_str);
            0
        });

        let meta = extract_placeholders(comment_regex, line).unwrap_or_default();

        reminds.push(Remind {
            datetime,
            message: line.trim_start().to_string(),
            position: Position {
                file: entry_path.clone(),
                line: line_num,
            },
            meta,
        });
        Ok(true)
    })
}

pub fn datetime_format_to_regex(format: &str) -> String {
    let mut re = format.to_string();
    let replacements = vec![
        ("%Y", r"\d{4}"),
        ("%m", r"\d{2}"),
        ("%d", r"\d{2}"),
        ("%H", r"\d{2}"),
        ("%M", r"\d{2}"),
        ("%S", r"\d{2}"),
    ];

    for (fmt, rep) in replacements {
        re = re.replace(fmt, rep);
    }

    re
}
