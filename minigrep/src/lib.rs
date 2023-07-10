use std::{fs, error::Error, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    let search = if config.ignore_case {search_case_insensitive} else {search_case_sensitive};

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("There is no query string!"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("There is no file for search"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query) )
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()) )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_parse_is_working () {
        let file = "file".to_owned();
        let query: String = "query".to_owned(); 

        let test_subject = vec!["path".to_owned(), query.clone(), file.clone()];

        let config = Config::build(test_subject.into_iter()).unwrap();

        let expect = Config {
            file_path: file,
            query: query,
            ignore_case: false
        };

        assert_eq!(config.file_path, expect.file_path);
        assert_eq!(config.query, expect.query);
        assert_eq!(config.ignore_case, expect.ignore_case);

    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
