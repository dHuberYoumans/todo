pub fn parse_task(task: &str) -> (String, String) {
    let (title, rest) = task.split_once('\n').unwrap_or((task, ""));
    let message = rest
        .lines()
        .skip_while(|line| line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    (title.to_string(), message)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_parse_task() {
        let task = "Title\nMessage";
        let expected_title = "Title".to_string();
        let expected_message = "Message".to_string();
        let parsed_task = parse_task(task);
        let (title, message) = parsed_task;
        assert_eq!(title, expected_title);
        assert_eq!(message, expected_message);

        let task = "Title";
        let expected_title = "Title".to_string();
        let expected_message = "".to_string();
        let parsed_task = parse_task(task);
        let (title, message) = parsed_task;
        assert_eq!(title, expected_title);
        assert_eq!(message, expected_message);

        let task = "Title\n\nParagraph1\nParagraph2";
        let expected_title = "Title".to_string();
        let expected_message = "Paragraph1\nParagraph2".to_string();
        let parsed_task = parse_task(task);
        let (title, message) = parsed_task;
        assert_eq!(title, expected_title);
        assert_eq!(message, expected_message);
    }
}
