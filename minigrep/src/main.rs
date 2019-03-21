use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        //eprintln prints to the command line instead of the redirect file if any
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //println!("Query: {:?} File: {:?}", query, filename);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application err: {}", e);
        process::exit(1);
    };
}