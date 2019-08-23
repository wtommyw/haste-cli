use std::fs;
use std::error::Error;
use std::env;

pub struct Options {
    pub filename: String,
    pub url: String
}

impl Options {
    pub fn new(args: &[String]) -> Result<Options, &'static str> {
        if args.len() < 2 {
            return Err("Missing filename");
        }

        let filename = args[1].clone();

        let url = env::var("HASTE_URL").unwrap_or_else(|_err| {
            String::from("https://hasteb.in/documents")
        });

        Ok(Options{ filename, url })
    }
}

pub fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(options.filename)?;

    println!("text:\n{}", content);

    println!("\nUrl:\n{}", options.url);

    Ok(())
}
