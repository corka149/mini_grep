extern crate minigrep;

use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // A nonzero exit status is a convention to signal to the process that called our
        // program that our program ended with an error state.
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        process::exit(1);
    }
}
