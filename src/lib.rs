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

    println!("Link: {}", &url);

    Ok(())
}
