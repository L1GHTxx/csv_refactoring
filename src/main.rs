#![allow(unused)]
use std::env;
use std::process;

use csv_refactoring::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("ඞ Problem parsing arguments: {} ඞ", err);
        process::exit(1);
    });

    if let Err(e) = csv_refactoring::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
