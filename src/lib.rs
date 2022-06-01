use anyhow::{bail, Context, Result};
use std::env;
use tokio::fs;

pub struct Config {
    query:          String,
    filename:       String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(
        mut args: impl ExactSizeIterator<Item = String>,
    ) -> Result<Config> {
        if args.len() != 2 {
            bail!("need 2 arguments");
        }

        Ok(Config {
            query:          args.next().unwrap(),
            filename:       args.next().unwrap(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub async fn run(config: Config) -> Result<()> {
    let lines = fs::read_to_string(&config.filename)
        .await
        .with_context(|| format!("reading file[{}]", config.filename))?;

    let matches = if config.case_sensitive {
        search_lines(&config.query, &lines)
    } else {
        search_lines_case_insensitive(&config.query, &lines)
    };
    for line in matches {
        println!("{}", line);
    }
    Ok(())
}

pub fn search_lines<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.contains(query))
        .collect()
}

pub fn search_lines_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|&line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{search_lines, search_lines_case_insensitive};

    const TEST_DOCS: &str = r#"Rust:
safe, fast, productive.
Pick three."#;

    #[test]
    fn test_search_lines() {
        let query = "duct";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_lines(query, TEST_DOCS)
        );
    }

    #[test]
    fn test_search_lines_case_insensitive() {
        let query = "DUCT";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_lines_case_insensitive(query, TEST_DOCS)
        );
    }
}
