use std::{env::{self, args}, fs};

fn main() {
    let args: Vec<String>= env :: args().collect();
let config =Config::new(&args);

    let content= fs::read_to_string(config.filename).expect(" somthing went wrong");
    println!("{}",content);
}

struct Config{
    query:String,
    filename:String,

}
impl Config
{fn new(args:&[String])->Config{

    let query= args[1].clone();
    let filename=args[2].clone();
     Config {query, filename}
    
    }
}
