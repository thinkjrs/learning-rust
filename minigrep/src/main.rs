use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for '{}' in file '{}'\n",
        config.query, config.file_path
    );

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 {
            let query = args[1].clone();
            let file_path = args[2].clone();

            return Ok(Config { query, file_path });
        }
        return Err("Not enough arguments!");
    }
}
