use crate::application::config::Config;

pub fn expand_alias(args: Vec<String>, config: &Config) -> Vec<String> {
    match args.as_slice() {
        [program, cmd, rest @ ..] => {
            match config.aliases.as_ref().and_then(|alias| alias.get(cmd)) {
                Some(alias) => {
                    let mut result: Vec<String> =
                        Vec::with_capacity(1 + alias.split_whitespace().count() + rest.len());
                    result.push(program.clone());
                    result.extend(alias.split_whitespace().map(String::from));
                    result.extend(rest.iter().cloned());
                    result
                }
                None => args,
            }
        }
        _ => args,
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::application::config::{Database, Style};
    use std::collections::HashMap;

    #[test]
    fn should_return_input_if_no_alias_is_present() {
        let config = Config {
            database: Database {
                todo_db: String::new(),
            },
            style: Style {
                id_length: 1,
                due_date_display_format: String::new(),
                due_date_input_format: String::new(),
                show_due: true,
                show_tag: true,
                sort_by: String::new(),
                table: String::new(),
            },
            aliases: None,
        };
        let args: Vec<String> = vec!["program".into(), "add --due=today".into()];
        let expanded = expand_alias(args, &config);
        let expected: Vec<String> = vec!["program".into(), "add --due=today".into()];
        assert_eq!(expanded, expected);
    }

    #[test]
    fn should_expand_alias() {
        let mut aliases = HashMap::new();
        aliases.insert("p1".to_string(), "add --prio=p1".to_string());
        let config = Config {
            database: Database {
                todo_db: String::new(),
            },
            style: Style {
                id_length: 1,
                due_date_display_format: String::new(),
                due_date_input_format: String::new(),
                show_due: true,
                show_tag: true,
                sort_by: String::new(),
                table: String::new(),
            },
            aliases: Some(aliases),
        };
        let args: Vec<String> = vec!["program".into(), "p1".into(), "--due=today".into()];
        let expanded = expand_alias(args, &config);
        let expected = vec!["program", "add", "--prio=p1", "--due=today"];
        assert_eq!(expanded, expected);
    }
}
