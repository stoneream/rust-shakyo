extern crate chapter12;

use std::{env, process};

use chapter12::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = chapter12::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
