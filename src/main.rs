use std::{
    env::{self, args},
    fmt::Error,
    fs, process,
};

use std::result;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("error msg{}", err);
        process::exit(1);
    });

    let content = fs::read_to_string(config.filename).expect(" somthing went wrong");
    println!("{}", content);
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("no enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
