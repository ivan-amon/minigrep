use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = match Config::build(&args) {
        Ok(conf) => conf,
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    
    Ok(())
}
