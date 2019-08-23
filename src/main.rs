use std::env;
use std::process;

use hst::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options: Options = Options::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = hst::run(options) {
        println!("Application error: {}", e);

        process::exit(1);
    };

}
