mod check_file;
mod constants;
mod draw_table;
mod utils;
use crate::check_file::CheckFile;
use std::error::Error;
use std::process;

pub struct Config {
    pub file_paths: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            eprintln!("Not enough arguments");
            std::process::exit(1);
        }
        let file_paths = args[1..].to_vec();
        Ok(Config { file_paths })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_paths = config.file_paths;
    if let Err(e) = CheckFile::run(file_paths) {
        println!("Application error: {e}");
        process::exit(1);
    }
    Ok(())
}
