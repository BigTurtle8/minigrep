use std::env;
use std::error::Error;
use std::fs;

pub mod search_engine;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("no file path string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_engine::search_case_insensitive(&config.query, &contents)
    } else {
        search_engine::search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
