use crate::consts::{DATA_DIR_NAME, DICT_FILE_NAME};
use crate::utils::CeOptions;
use regex::Regex;
use std::fs;
use std::path::Path;
use crate::utils::*;
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref CEDICT_LINE: Regex = Regex::new(r"^(?P<simplified>\S+)\s+(?P<traditional>\S+)\s+(?P<pinyin>\[[^]]+\])\s+(?P<definitions>/.*/)$").unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CeLine {
    pub simplified: String,
    pub traditional: String,
    pub pinyin: Vec<String>,
    pub definitions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CeResult {
    pub lines: Vec<CeLine>
}

pub struct CeParser {
    base_path: String,
    data_dir: String,
    dict_file_path: String,
}

impl CeParser {
    pub fn init() -> CeParser {
        let base_path = String::from("./");
        let data_dir = Path::new(&base_path).join(DATA_DIR_NAME);
        let dict_file_path = Path::new(&base_path).join(DATA_DIR_NAME).join(DICT_FILE_NAME);

        CeParser {
            base_path: base_path.to_string(),
            data_dir: data_dir.into_os_string().into_string().unwrap(),
            dict_file_path: dict_file_path.into_os_string().into_string().unwrap(),
        }
    }

    pub fn init_with_options(opts: &CeOptions) -> CeParser {
        let base_path = opts.base_path.clone();
        let data_dir = Path::new(&base_path).join(DATA_DIR_NAME);
        let dict_file_path = Path::new(&base_path).join(DATA_DIR_NAME).join(DICT_FILE_NAME);
        let data_dir_string = data_dir.into_os_string().into_string().unwrap();
        
        if !Path::new(&base_path).join(DATA_DIR_NAME).exists() {
            create_directory(&data_dir_string)
        }

        CeParser {
            base_path: base_path.to_string(),
            data_dir: (&data_dir_string).clone(),
            dict_file_path: dict_file_path.into_os_string().into_string().unwrap(),
        }
    }

    pub fn parse(&self) -> CeResult {
        let mut results = Vec::<CeLine>::new();
        let data = fs::read_to_string(&self.dict_file_path).expect("Unable to read file");
        let lines = data.split("\r\n").collect::<Vec<&str>>();
        let mut counter = 0;
        let lines_length = &lines.len();
        for line in &lines {
            counter += 1;
            match CeParser::parse_line(line) {
                Some(line) => results.push(line),
                None => continue,
            }
            if counter % 10000 == 0 {
                println!("{}/{} lines read", counter, lines_length);
            }
        }
        CeResult {
            lines: results
        }
    }

    fn parse_line(line: &str) -> Option<CeLine> {
        if line.len() > 0 && line.chars().nth(0).unwrap() == '#' {
            return None;
        }

        let splitted = line.split_whitespace().collect::<Vec<&str>>();
        if splitted.len() <= 0 {
            return None;
        }

        let re = &CEDICT_LINE;
        let captures = re.captures(line);
        match captures {
            Some(captures) => {
                let simplified = captures["simplified"].trim().to_string();
                let traditional = captures["traditional"].trim().to_string();
                let pinyin = captures["pinyin"]
                    .trim_end_matches("]")
                    .trim_start_matches("[")
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                let definitions = captures["definitions"]
                    .split("/")
                    .map(|x| x.to_string())
                    .filter(|x| x != "")
                    .collect::<Vec<String>>();
        
                let translated_pinyins = pinyin; 
                // CeParser::translate_pinyin(pinyin);
                let result = CeLine {
                    simplified: simplified,
                    traditional: traditional,
                    pinyin: translated_pinyins,
                    definitions: definitions,
                };
        
                Some(result)
            },
            None => None
        }
    }

    fn translate_pinyin(pinyins: Vec<String>) -> Vec<String> {
        let mut result = Vec::<String>::new();
        for p in pinyins {
            let pinyin_last = p.len() - 1;
            let last_char = p.chars().nth(pinyin_last).unwrap();
            if last_char.is_digit(10) {
                let char_digit = last_char.to_digit(10).unwrap();
                for c in p.chars().rev() {
                    let _new_char = match c { 
                        'a' => match char_digit {
                            _ => 'a'
                        },
                        'e' => match char_digit {
                            _ => 'e'
                        },
                        'i' => match char_digit {
                            _ => 'i'
                        },
                        'o' => match char_digit {
                            _ => 'o'
                        },
                        'u' => match char_digit {
                            _ => 'u'
                        },
                        _ => '_'
                    };
                }
            } else {
                result.push(p);
            }
        }
        result
    }
}
