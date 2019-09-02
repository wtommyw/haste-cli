use regex::Regex;

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
        if Options::is_url(&args[1]) {
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

    pub fn is_url(str: &String) -> bool {
        let regex = Regex::new(r"(https|http)(://)[\w]*(\.)[\w]*(/){0,1}").unwrap_or_else(|e| {
            panic!("Could not compile regex: {}", e)
        });

        match regex.find(str) {
            Some(_) => return true,
            None => return false
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static DEFAULT_URL: &str = "https://pastie.io/documents";

    fn create_upload_start_arguments() -> [String; 2] {
        [String::from("command"), String::from("./test-data/test.txt")]

    }

    fn create_custom_upload_start_arguments(url: &String) -> [String; 3] {
        [String::from("command"), String::from("./test-data/test.txt"), url.clone()]

    }

    fn create_download_start_arguments() -> [String; 3] {
        [String::from("command"), String::from("https://pastie.io/documents"), String::from("./test-data/test.txt")]

    }

    #[test]
    fn is_url_returns_true_on_https_url() {
        let url = String::from("https://pastie.io/documents");

        assert_eq!(Options::is_url(&url), true);
    }

    #[test]
    fn is_url_returns_true_on_http_url() {
        let url = String::from("http://pastie.io/documents");

        assert_eq!(Options::is_url(&url), true);
    }

    #[test]
    fn is_url_returns_false_on_filename() {
        let filename = String::from("./test-data/test.txt");

        assert_eq!(Options::is_url(&filename), false);
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
        let filename = String::from("./test-data/test.txt");

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
}
