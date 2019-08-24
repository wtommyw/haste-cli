extern crate reqwest;

use std::fs::File;
use std::error::Error;
use std::env;
use std::collections::HashMap;

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

pub fn post_data(options: &Options, file: File) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let body = reqwest::Body::new(file);

    let res: HashMap<String, String> = client.post(&options.url)
        .body(body)
        .send()?.json()?;

    let _key = res.get("key").unwrap().clone();

    Ok(_key)
}

pub fn create_share_link(base_url: &String, key: &String) -> String {
    let base = &base_url.clone();

    base.replace("documents", &key)

}

pub fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let file  = File::open(&options.filename)?;

    let key = post_data(&options, file).unwrap_or_else(|err| {
        panic!("Could not POST: {:?}", err)
    });

    let url = create_share_link(&options.url, &key);

    println!("Uploaded {} to:\n{}", &options.filename, &url);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_start_arguments() -> [String; 2] {
        let program_name = String::from("Testing");
        let filename = String::from("test.txt");

        [program_name.clone(), filename.clone()]

    }

    #[test]
    fn options_constructor_gives_error_no_filename() {
        let args: [String; 1] = [String::from("PogramName")];

        match Options::new(&args) {
            Ok(_) => assert!(false, "Only 1 arg (program name) was given, an error should be returned."),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn options_constructor_returns_options () {
        let filename = String::from("test.txt");

        let args: [String; 2] = create_start_arguments();

        match Options::new(&args) {
            Ok(options) => assert_eq!(options.filename, filename),
            Err(_) => assert!(false, "Filename was given, method should not fail.")
        }

    }

    #[test]
    fn options_constructor_uses_standard_url_without_env_var() {

        let default_url = String::from("https://hasteb.in/documents");
        env::remove_var("HASTE_URL");

        let args: [String; 2] = create_start_arguments();

        match Options::new(&args) {
            Ok(options) => assert_eq!(options.url, default_url),
            Err(_) => assert!(false, "Filename was given, method should not fail.")
        }

    }

    #[test]
    fn options_constructor_uses_custom_url_from_env_var() {

        let custom_url = String::from("https://pastie.io/documents");
        env::set_var("HASTE_URL", &custom_url);

        let args: [String; 2] = create_start_arguments();

        match Options::new(&args) {
            Ok(options) => assert_eq!(options.url, custom_url),
            Err(_) => assert!(false, "Filename was given, method should not fail.")
        }

    }

}
