use regex::{Captures, Regex};

pub fn parse_task(task: &str) -> (String, String) {
    let (title, rest) = task.split_once('\n').unwrap_or((task, ""));
    let message = rest
        .lines()
        .skip_while(|line| line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    (title.to_string(), message)
}

pub fn prettify(haystack: &str) -> String {
    // haystack = [text](link)
    let re_links = Regex::new(r"\[(?P<text>[^\]]+)\]\((?P<url>[^)]+)\)").unwrap();
    let re_box = Regex::new(r"(?m)^(?P<indent>[ \t]*)-\s*\[\s\]\s*(?P<text>.+)$").unwrap();
    let re_checked_box = Regex::new(r"(?m)^(?P<indent>[ \t]*)-\s*\[x\]\s*(?P<text>.+)$").unwrap();
    let re_bullets = Regex::new(r"(?m)^(?P<indent>[ \t]*)-\s+(?P<text>.+)$").unwrap();
    let re_quote = Regex::new(r"(?m)^(?P<quote>[ \t]*)>\s*(?P<text>.+)$").unwrap();
    let re_italic = Regex::new(r"_(?P<italic>[^_\n]+)_").unwrap();
    let re_bold = Regex::new(r"\*(?P<bold>[^*\n]+)\*").unwrap();
    let output = re_links.replace_all(haystack, |caps: &Captures| {
        let text = &caps["text"];
        let url = &caps["url"];
        link(text, url)
    });
    let output = re_checked_box.replace_all(&output, "$indent ⏹ $text");
    let output = re_box.replace_all(&output, "$indent □ $text");
    let output = re_bullets.replace_all(&output, "$indent • $text");
    let output = re_quote.replace_all(&output, "$indent ┃ $text");
    let output = re_italic.replace_all(&output, "\x1b[3m$italic\x1b[0m");
    let output = re_bold.replace_all(&output, "\x1b[1m$bold\x1b[0m");
    output.to_string()
}

pub fn link(text: &str, url: &str) -> String {
    let url = if url.contains("://") {
        url
    } else {
        &format!("https://{}", url)
    };
    format!(
        "\u{1b}[34m↗\u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\\u{1b}[0m",
        url, text,
    )
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
