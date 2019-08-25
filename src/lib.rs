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
            String::from("https://pastie.io/documents")
        });

        Ok(Options{ filename, url })
    }
}

pub fn post_data(options: &Options, file: File) -> Result<reqwest::Response, &'static str> {
    let client = reqwest::Client::new();
    let body = reqwest::Body::new(file);

    let res = match client.post(&options.url).body(body).send() {
        Ok(response) => response,
        Err(err) => {
            if err.is_http() {
                return Err("Could not POST");
            } else {
                return Err("Invalid URL");
            }
        }
    };

    if res.status().is_success() {
        return Ok(res);
    } else {
        return Err("POST was unsuccessful");
    }
}

pub fn parse_response(response: &mut reqwest::Response) -> String {
    let body: HashMap<String, String> = response.json().unwrap();

    body.get("key").unwrap().clone()
}

pub fn create_share_link(base_url: &String, key: &String) -> String {
    let base = &base_url.clone();

    base.replace("documents", &key)

}

pub fn run(options: Options) -> Result<(), Box<dyn Error>> {
    let file  = File::open(&options.filename)?;

    let mut response = post_data(&options, file).unwrap_or_else(|err| {
        panic!("Could not POST: {:?}", err)
    });

    let key = parse_response(&mut response);

    let url = create_share_link(&options.url, &key);

    println!("Uploaded {} to:\n{}", &options.filename, &url);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    static DEFAULT_URL: &str = "https://pastie.io/documents";

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
        env::remove_var("HASTE_URL");

        let default_url = String::from(DEFAULT_URL);
        let args: [String; 2] = create_start_arguments();

        match Options::new(&args) {
            Ok(options) => assert_eq!(options.url, default_url),
            Err(_) => assert!(false, "Filename was given, method should not fail.")
        }

    }

    #[test]
    fn options_constructor_uses_custom_url_from_env_var() {
        let custom_url = String::from("https://hasteb.in/documents");
        env::set_var("HASTE_URL", &custom_url);

        let args: [String; 2] = create_start_arguments();

        match Options::new(&args) {
            Ok(options) => assert_eq!(options.url, custom_url),
            Err(_) =>  assert!(false, "Filename was given, method should not fail.")
        }

    }

    #[test]
    fn post_data_to_invalid_url_returns_error() {
        let custom_url = String::from("invalid_url");
        env::set_var("HASTE_URL", &custom_url);

        let args: [String; 2] = create_start_arguments();
        let options = Options::new(&args).unwrap();
        let file  = File::open(&options.filename).unwrap();

        match post_data(&options, file) {
            Ok(_) => assert!(false, "This method is designed to fail"),
            Err(err) => assert_eq!("Invalid URL", err)

        }
    }

    #[test]
    fn post_dat_to_unreachable_url_returns_error() {

        let _m = mock("POST", "/documents").with_status(501);
        let mut custom_url = String::from(&mockito::server_url());
        custom_url.push_str("/documents");

        env::set_var("HASTE_URL", &custom_url);

        let args: [String; 2] = create_start_arguments();
        let options = Options::new(&args).unwrap();
        let file  = File::open(&options.filename).unwrap();

        match post_data(&options, file) {
            Ok(_t) => assert!(false, "Method should fail on unsuccessful response"),
            Err(err) => assert_eq!("POST was unsuccessful", err)
        };
    }

}
