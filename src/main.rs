#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod parser;
mod utils;
mod downloader;
mod consts;

use crate::downloader::{CeDownloader};
use crate::parser::{CeParser};
use crate::utils::{CeOptions};

fn main() {
    let opts = CeOptions { base_path: String::from("./") };
    let dw = CeDownloader::init_with_options(&opts);
    let _ = dw.download();
    let p = CeParser::init_with_options(&opts);
    let lines = p.parse();
    let json = serde_json::to_string_pretty(&lines).unwrap();
    utils::write_file(&String::from("./data/output.json"), &json);
}

#[cfg(test)]
mod tests {
    use crate::downloader::{CeDownloader};
    use crate::parser::{CeParser};
    use crate::utils::{CeOptions};

    #[test]
    fn download_dictionary() {
        let dw = CeDownloader::init();
        let _ = dw.download().unwrap();
    }

    #[test]
    fn download_dictionary_options() {
        let opts = CeOptions { base_path: String::from("./") };
        let dw = CeDownloader::init_with_options(&opts);
        let _ = dw.download();
    }

    #[test]
    fn parse_dictionary() {
        let p = CeParser::init();
        p.parse();
    }

    #[test]
    fn parse_dictionary_options() {
        let opts = CeOptions { base_path: String::from("./") };
        let p = CeParser::init_with_options(&opts);
        p.parse();
    }
}