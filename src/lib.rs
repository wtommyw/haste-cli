use std::fs;
use std::error::Error;

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

        let url: String;
        if args.len() == 3 {
            url = args[2].clone();
        } else {
            url = String::from("https://hasteb.in/documents");
        }

        Ok(Options{ filename, url })
    }
}

pub fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(options.filename)?;

    println!("text:\n{}", content);

    println!("\nUrl:\n{}", options.url);

    Ok(())
}
