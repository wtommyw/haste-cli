extern crate reqwest;
extern crate regex;

use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use regex::Regex;
use reqwest::{Response, Client, Body};

pub struct Options {
    pub filename: String,
    pub url: String,
    pub mode: Mode
}

pub enum Mode {
    Upload,
    Download
}

impl Options {
    pub fn new(args: &[String]) -> Result<Options, &'static str> {
        if args.len() < 2 {
            return Err("Missing arguments");
        }

        let (filename, url, mode);
        if is_url(&args[1]) {
            mode = Mode::Download;
            url = args[1].clone();

            if args.len() < 3 {
                return Err("Missing filename");
            } else {
                filename = args[2].clone();
            }
        } else {
            mode = Mode::Upload;
            filename = args[1].clone();

            if args.len() < 3 {
                url = String::from("https://pastie.io/documents")
            } else {
                url = args[2].clone();
            }
        }

        Ok(Options{ filename, url, mode })
    }
}

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

pub fn is_url(str: &String) -> bool {
    let regex = Regex::new(r"(https|http)(://)[\w]*(\.)[\w]*(/){0,1}").unwrap_or_else(|e| {
        panic!("Could not compile regex: {}", e)
    });

    match regex.find(str) {
        Some(_) => return true,
        None => return false
    };


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

pub fn download(options: &Options) -> Result<(), &'static str> {
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

    fn create_upload_start_arguments() -> [String; 2] {
        let program_name = String::from("Testing");
        let filename = String::from("test.txt");

        [program_name.clone(), filename.clone()]

    }

    fn create_custom_upload_start_arguments(url: &String) -> [String; 3] {
        let program_name = String::from("Testing");
        let filename = String::from("test.txt");

        [program_name.clone(), filename.clone(), url.clone()]

    }

    fn create_download_start_arguments() -> [String; 3] {
        let program_name = String::from("Testing");
        let url = String::from("https://pastie.io/documents");
        let filename = String::from("test.txt");

        [program_name.clone(), url.clone(), filename.clone()]
    }

    #[test]
    fn is_url_returns_true_on_https_url() {
        let url = String::from("https://pastie.io/documents");

        assert_eq!(is_url(&url), true);
    }

    #[test]
    fn is_url_returns_true_on_http_url() {
        let url = String::from("http://pastie.io/documents");

        assert_eq!(is_url(&url), true);
    }

    #[test]
    fn is_url_returns_false_on_filename() {
        let filename = String::from("test.txt");

        assert_eq!(is_url(&filename), false);
    }

    #[test]
    fn options_constructor_gives_error_no_arguments() {
        let args: [String; 1] = [String::from("PogramName")];

        match Options::new(&args) {
            Ok(_) => assert!(false, "Only 1 arg (program name) was given, an error should be returned."),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn options_constructor_returns_options () {
        let filename = String::from("test.txt");

        let args: [String; 2] = create_upload_start_arguments();

        match Options::new(&args) {
            Ok(options) => assert_eq!(options.filename, filename),
            Err(_) => assert!(false, "All arguments were given, method should not fail.")
        }

    }

    #[test]
    fn options_constructor_set_upload_mode_when_given_filename() {
        let args: [String; 2] = create_upload_start_arguments();

        match Options::new(&args) {
            Ok(options) => match options.mode {
                Mode::Download => assert!(false, "Filename was given, method should be upload"),
                Mode::Upload => assert!(true)
            },
            Err(_) =>  assert!(false, "All arguments were given, method should not fail.")
        }
    }

    #[test]
    fn options_constructor_uses_default_upload_url_when_not_give() {

        let args: [String; 2] = create_upload_start_arguments();

        match Options::new(&args) {
            Ok(options) => assert_eq!(options.url, DEFAULT_URL),
            Err(_) => assert!(false, "All arguments were given, method should not fail.")
        }
    }

    #[test]
    fn options_constructor_uses_custom_upload_url() {
        let custom_url = String::from("https://hasteb.in/documents");

        let args: [String; 3] = create_custom_upload_start_arguments(&custom_url);

        match Options::new(&args) {
            Ok(options) => assert_eq!(options.url, custom_url),
            Err(_) => assert!(false, "All arguments were given, method should not fail.")
        }
    }

    #[test]
    fn options_constructor_set_download_mode_when_given_url() {
        let args: [String; 3] = create_download_start_arguments();

        match Options::new(&args) {
            Ok(options) => match options.mode {
                Mode::Download => assert!(true),
                Mode::Upload => assert!(false, "URL was given, method should be download")
            },
            Err(_) =>  assert!(false, "All arguments were given, method should not fail.")
        }
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
    fn post_dat_to_unreachable_url_returns_error() {

        let _m = mock("POST", "/documents").with_status(501);
        let mut custom_url = String::from(&mockito::server_url());
        custom_url.push_str("/documents");

        let args: [String; 3] = create_custom_upload_start_arguments(&custom_url);
        let options = Options::new(&args).unwrap();
        let file  = File::open(&options.filename).unwrap();

        match post_data(&options, file) {
            Ok(_t) => assert!(false, "Method should fail on unsuccessful response"),
            Err(err) => assert_eq!("POST was unsuccessful", err)
        };
    }

    #[test]
    fn create_share_link_changes_documents_to_key() {
        let key = String::from("abcdef");
        let wanted_url = String::from("https://pastie.io/abcdef");
        let default_url = String::from(DEFAULT_URL);

        assert_eq!(create_share_link(&default_url, &key), wanted_url);

    }

}
