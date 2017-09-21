extern crate minigrep;

use minigrep::Config;
use std::env;
use std::process;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        // A nonzero exit status is a convention to signal to the process that called our
        // program that our program ended with an error state.
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
