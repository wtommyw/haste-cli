pub mod haste;

extern crate reqwest;
extern crate regex;
extern crate serde;

use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use reqwest::blocking::{Response, Client};
pub use crate::haste::options::{Options, Mode};

pub fn post_data(options: &Options, file: File) -> Result<Response, &'static str> {
    let client = Client::new();

    let res = match client.post(&options.url).body(file).send() {
        Ok(response) => response,
        Err(err) => {
            if err.is_status() {
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

pub fn parse_response(response: Response) -> Result<String, Box<dyn Error>> {
    let response_body = response.json::<HashMap<String, String>>();

    if response_body.is_ok() {
        let body = response_body.unwrap();
        Ok(body.get("key").unwrap().clone())
    } else {
        Err(Box::new(response_body.unwrap_err()))
    }
    // let body: HashMap<String, String> = response.json();

    
}

pub fn create_share_link(base_url: &String, key: &String) -> String {
    let base = &base_url.clone();

    base.replace("documents", &key)

}

pub fn upload(options: &Options) -> Result<(), Box<dyn Error>> {
    let file  = File::open(&options.filename)?;

    let response = post_data(&options, file).unwrap_or_else(|err| {
        panic!("Could not POST: {:?}", err)
    });

    let parsed_respone = parse_response(response);
    if parsed_respone.is_ok() {
        let url = create_share_link(&options.url, &parsed_respone.unwrap());

        println!("Uploaded {} to:\n{}", &options.filename, &url);

        Ok(())
    } else {
        Err(parsed_respone.unwrap_err())
    }
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
    fn parse_response_returns_key_from_succesful_post() {
        let key = "rAnd0mK3y";
        let _m2 = mock("POST", "/documents").with_body("{\"key\": \"rAnd0mK3y\"}").create();
        let mut custom_url = String::from(&mockito::server_url());
        custom_url.push_str("/documents");

        let args: [String; 3] = create_custom_upload_start_arguments(&custom_url);
        let options = Options::new(&args).unwrap();
        let file  = File::open(&options.filename).unwrap();

        let response: Response = post_data(&options, file).unwrap_or_else(|e| {
            panic!("Succesfull POST test method failed: {:?}", e)
        });

        let parsed_response = parse_response(response);

        assert!(parsed_response.is_ok(), "parse_response should return an OK with a valid JSON body");
        assert_eq!(parsed_response.unwrap(), key);
    }

    #[test]
    fn parse_response_returns_error_on_invalid_json_body()  {
        let _m3 = mock("POST", "/documents").with_body("\"key\" \"rAnd0mK3y\"").create();
        let mut custom_url = String::from(&mockito::server_url());
        custom_url.push_str("/documents");

        let args: [String; 3] = create_custom_upload_start_arguments(&custom_url);
        let options = Options::new(&args).unwrap();
        let file  = File::open(&options.filename).unwrap();

        let response: Response = post_data(&options, file).unwrap_or_else(|e| {
            panic!("Succesfull POST test method failed: {:?}", e)
        });

        let parsed_response = parse_response(response);
        assert!(parsed_response.is_err(), "parse_response should return an error with a invalid JSON body")
    }

    #[test]
    fn create_share_link_changes_documents_to_key() {
        let key = String::from("abcdef");
        let wanted_url = String::from("https://pastie.io/abcdef");
        let default_url = String::from(DEFAULT_URL);

        assert_eq!(create_share_link(&default_url, &key), wanted_url);

    }

}
