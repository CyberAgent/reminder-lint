use std::collections::HashMap;

use regex::Regex;

fn convert_pattern_to_regex(pattern: &str) -> String {
    let placeholder_re = Regex::new(r"\$\{(\w+)\}").unwrap();
    placeholder_re
        .replace_all(pattern, "(?P<$1>.*)")
        .to_string()
}

pub(crate) fn extract_placeholders(pattern: &str, text: &str) -> Option<HashMap<String, String>> {
    let re_pattern = convert_pattern_to_regex(pattern);
    let re = Regex::new(&re_pattern).unwrap();

    re.captures(text).map(|captures| {
        let mut map = HashMap::new();
        for name in re.capture_names().flatten() {
            if let Some(value) = captures.name(name) {
                map.insert(name.to_string(), value.as_str().to_string());
            }
        }
        map
    })
}

pub(crate) fn convert_meta_regex(reg_str: &str) -> String {
    let meta_regex = Regex::new(r"\$\{(\w+)\}").unwrap();
    let reg = meta_regex.replace_all(reg_str, "(.*)").to_string();

    reg
}

#[cfg(test)]
include!("./meta_test.rs");