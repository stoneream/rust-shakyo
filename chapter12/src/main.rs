extern crate chapter12;

use std::{env, process};

use chapter12::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = chapter12::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
