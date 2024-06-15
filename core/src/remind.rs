use anyhow::Error;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use grep_regex::RegexMatcherBuilder;
use grep_searcher::sinks::UTF8;
use grep_searcher::SearcherBuilder;
use ignore::WalkBuilder;
use regex::RegexBuilder;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct Remind {
    pub datetime: i64,
    pub message: String,
    pub position: Position,
    pub meta: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Position {
    pub file: String,
    pub line: u64,
}

pub fn list_reminders(
    comment_regex: &str,
    datetime_format: &str,
    ignore_config_file: &str,
    search_directory: &str,
) -> Result<Vec<Remind>, Error> {
    let matcher = RegexMatcherBuilder::new().build(comment_regex)?;

    let mut searcher_builder = SearcherBuilder::new();
    let mut searcher = searcher_builder
        .binary_detection(grep_searcher::BinaryDetection::quit(b'\0'))
        .line_number(true)
        .build();

    let mut builder = WalkBuilder::new(search_directory);
    let walker = builder
        .hidden(false)
        .add_custom_ignore_filename(ignore_config_file)
        .ignore(true)
        .parents(false)
        .build();

    let datetime_regex = datetime_format_to_regex(datetime_format);
    let datetime_regex = RegexBuilder::new(&datetime_regex).build()?;
    let reminds = walker
        .filter_map(|e| {
            let mut reminders: Vec<Remind> = vec![];
            let entry = e.ok()?;
            let _result = searcher.search_path(
                &matcher,
                entry.path(),
                line_processor(
                    &mut reminders,
                    entry.path().display().to_string(),
                    datetime_format.to_owned(),
                    &datetime_regex,
                ),
            );
            Some(reminders)
        })
        .flatten()
        .collect();

    Ok(reminds)
}

fn parse_datetime(v: &str, format: &str) -> Result<i64, Error> {
    if format.contains("%H") || format.contains("%M") || format.contains("%S") {
        let datetime = NaiveDateTime::parse_from_str(&v, &format)?;
        Ok(datetime.and_utc().timestamp())
    } else {
        let date = NaiveDate::parse_from_str(&v, &format)?;
        Ok(date.and_time(NaiveTime::default()).and_utc().timestamp())
    }
}

fn line_processor<'a>(
    reminds: &'a mut Vec<Remind>,
    entry_path: String,
    datetime_format: String,
    datetime_regex: &'a regex::Regex,
) -> UTF8<impl FnMut(u64, &str) -> Result<bool, io::Error> + 'a> {
    UTF8(move |line_num, line| match datetime_regex.find(line) {
        Some(m) => {
            let datetime_str = m.as_str();
            let parsed = parse_datetime(&datetime_str, &datetime_format);
            let datetime = parsed.unwrap_or_else(|e| {
                eprintln!("Error parsing datetime: {}", e);
                0
            });
            reminds.push(Remind {
                datetime,
                message: line.trim_start().to_string(),
                position: Position {
                    file: entry_path.clone(),
                    line: line_num.into(),
                },
                meta: HashMap::new(),
            });
            Ok(true)
        }
        None => Ok(false),
    })
}

fn datetime_format_to_regex(format: &str) -> String {
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
