pub mod haste;

extern crate reqwest;
extern crate regex;

use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use reqwest::{Response, Client, Body};
pub use crate::haste::options::{Options, Mode};

pub fn post_data(options: &Options, file: File) -> Result<Response, &'static str> {
    let client = Client::new();
    let body = Body::new(file);

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

pub fn parse_response(response: &mut Response) -> String {
    let body: HashMap<String, String> = response.json().unwrap();

    body.get("key").unwrap().clone()
}

pub fn create_share_link(base_url: &String, key: &String) -> String {
    let base = &base_url.clone();

    base.replace("documents", &key)

}

pub fn upload(options: &Options) -> Result<(), Box<dyn Error>>{
    let file  = File::open(&options.filename)?;

    let mut response = post_data(&options, file).unwrap_or_else(|err| {
        panic!("Could not POST: {:?}", err)
    });

    let key = parse_response(&mut response);

    let url = create_share_link(&options.url, &key);

    println!("Uploaded {} to:\n{}", &options.filename, &url);

    Ok(())
}

pub fn download(_options: &Options) -> Result<(), &'static str> {
    Err("To be implemented")
}

pub fn run(options: Options) -> Result<(), Box<dyn Error>> {
    match options.mode {
        Mode::Upload => upload(&options)?,
        Mode::Download => download(&options)?
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    static DEFAULT_URL: &str = "https://pastie.io/documents";

    fn create_custom_upload_start_arguments(url: &String) -> [String; 3] {
        [String::from("command"), String::from("./test-data/test.txt"), url.clone()]

    }

    #[test]
    fn post_data_to_invalid_url_returns_error() {
        let custom_url = String::from("invalid_url");

        let args: [String; 3] = create_custom_upload_start_arguments(&custom_url);
        let options = Options::new(&args).unwrap();
        let file  = File::open(&options.filename).unwrap();

        match post_data(&options, file) {
            Ok(_) => assert!(false, "This method is designed to fail"),
            Err(err) => assert_eq!("Invalid URL", err)

        }
    }

    #[test]
    fn post_data_to_unreachable_url_returns_error() {
        let _m = mock("POST", "/documents").with_status(500).create();
        let mut custom_url = String::from(&mockito::server_url());
        custom_url.push_str("/documents");

        let args: [String; 3] = create_custom_upload_start_arguments(&custom_url);
        let options = Options::new(&args).unwrap();
        let file  = File::open(&options.filename).unwrap();

        match post_data(&options, file) {
            Ok(_) => assert!(false, "Method should fail on unsuccessful response"),
            Err(err) => assert_eq!("POST was unsuccessful", err)
        };
    }

    #[test]
    fn parse_response_returns_key_from_succesfull_post() {
        let key = "rAnd0mK3y";
        let _m2 = mock("POST", "/documents").with_body("{\"key\": \"rAnd0mK3y\"}").create();
        let mut custom_url = String::from(&mockito::server_url());
        custom_url.push_str("/documents");

        let args: [String; 3] = create_custom_upload_start_arguments(&custom_url);
        let options = Options::new(&args).unwrap();
        let file  = File::open(&options.filename).unwrap();

        let mut response: Response = post_data(&options, file).unwrap_or_else(|e| {
            panic!("Succesfull POST test method failed: {:?}", e)
        });

        assert_eq!(parse_response(&mut response), key);

    }

    #[test]
    fn create_share_link_changes_documents_to_key() {
        let key = String::from("abcdef");
        let wanted_url = String::from("https://pastie.io/abcdef");
        let default_url = String::from(DEFAULT_URL);

        assert_eq!(create_share_link(&default_url, &key), wanted_url);

    }

}
