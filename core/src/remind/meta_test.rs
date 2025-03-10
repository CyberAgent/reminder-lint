mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_pattern_to_regex() {
        let pattern = r"@${assignee} remind:\W?";
        let expected = r"@(?P<assignee>.*) remind:\W?";
        assert_eq!(convert_pattern_to_regex(pattern), expected);
    }

    #[test]
    fn test_convert_pattern_to_regex_multiple_placeholders() {
        let pattern = r"@${assignee} remind: ${task}\W?";
        let expected = r"@(?P<assignee>.*) remind: (?P<task>.*)\W?";
        assert_eq!(convert_pattern_to_regex(pattern), expected);
    }

    #[test]
    fn test_extract_placeholders() {
        let pattern = r"@${assignee} remind:\W?";
        let text = "@alice remind: TODO";
        let expected = Some(HashMap::from([(
            "assignee".to_string(),
            "alice".to_string(),
        )]));

        assert_eq!(extract_placeholders(pattern, text), expected);
    }

    #[test]
    fn test_multiple_extract_placeholders() {
        let pattern = r"@${assignee} remind: ${task}\W?";
        let text = "@alice remind: TODO";
        let expected = Some(HashMap::from([
            ("assignee".to_string(), "alice".to_string()),
            ("task".to_string(), "TODO".to_string()),
        ]));

        assert_eq!(extract_placeholders(pattern, text), expected);
    }

    #[test]
    fn test_convert_meta_regex() {
        let reg_str = r"@${assignee} remind:\W?";
        let expected = r"@(.*) remind:\W?";
        assert_eq!(convert_meta_regex(reg_str), expected);
    }

    #[test]
    fn test_convert_meta_regex_multiple_placeholders() {
        let reg_str = r"@${assignee} remind: ${task}\W?";
        let expected = r"@(.*) remind: (.*)\W?";
        assert_eq!(convert_meta_regex(reg_str), expected);
    }

    #[test]
    fn test_native_name_extract_placeholders() {
        let pattern = r"@(?P<assignee>.*) remind:\W?";
        let text = "@alice remind: TODO";
        let expected = Some(HashMap::from([(
            "assignee".to_string(),
            "alice".to_string(),
        )]));

        assert_eq!(extract_placeholders(pattern, text), expected);
    }

    #[test]
    fn test_native_name_extract_placeholders_multiple() {
        let pattern = r"@(?P<assignee>.*) remind: (?P<task>.*)\W?";
        let text = "@alice remind: TODO";
        let expected = Some(HashMap::from([
            ("assignee".to_string(), "alice".to_string()),
            ("task".to_string(), "TODO".to_string()),
        ]));

        assert_eq!(extract_placeholders(pattern, text), expected);
    }

    #[test]
    fn test_native_name_extract_placeholders_with_special_names() {
        let pattern = r"@(?P<user_1>.*)_remind_(?P<task2>\w+):\W?";
        let text = "@bob_remind_cleanRoom:";
        let expected = Some(HashMap::from([
            ("user_1".to_string(), "bob".to_string()),
            ("task2".to_string(), "cleanRoom".to_string()),
        ]));

        assert_eq!(extract_placeholders(pattern, text), expected);
    }

    #[test]
    fn test_native_name_no_match() {
        let pattern = r"@(?P<assignee>.*) remind:\W?";
        let text = "@bob notify: cleanRoom";
        let expected = None;

        assert_eq!(extract_placeholders(pattern, text), expected);
    }

    #[test]
    fn test_native_partial_match() {
        let pattern = r"Task: (?P<task>[^,]+), Assigned to: (?P<assignee>[\w\s]+)";
        let text = "Task: Refactor code, Assigned to: Alice, Due: tomorrow";
        let expected = Some(HashMap::from([
            ("task".to_string(), "Refactor code".to_string()),
            ("assignee".to_string(), "Alice".to_string()),
        ]));

        assert_eq!(extract_placeholders(pattern, text), expected);
    }

    #[test]
    fn test_native_name_extract_partial_placeholders() {
        let pattern = r"(@(?P<assignee>.+) )?remind: (?P<task>.*)\W?";
        let text = "remind: TODO";
        let expected = Some(HashMap::from([("task".to_string(), "TODO".to_string())]));

        assert_eq!(extract_placeholders(pattern, text), expected);
    }
}
