extern crate haste;

use std::env;
use std::process;

use haste::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options: Options = Options::new(&args).unwrap_or_else(|_err| {
        println!("Usage:");
        println!("Uploading a file:\nhaste <filename> [url]");
        println!("Downloading a file:\nhaste <url> <save to filename>");
        process::exit(1)
    });

    if let Err(e) = haste::run(options) {
        println!("Application error: {}", e);

        process::exit(1);
    };

}
