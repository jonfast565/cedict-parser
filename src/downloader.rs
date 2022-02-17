use crate::consts::{DATA_DIR_NAME, DICT_FILE_NAME, FILE_DOWNLOAD_LOCATION};
use crate::utils::CeOptions;
use flate2::read::GzDecoder;
use std::io::Read;
use std::path::Path;
use crate::utils::*;

pub struct CeDownloader {
    base_path: String,
    data_dir: String,
    dict_file_path: String,
}

impl CeDownloader {
    pub fn init() -> CeDownloader {
        let base_path = String::from("./");
        let data_dir = Path::new(&base_path).join(DATA_DIR_NAME);
        let dict_file_path = Path::new(&base_path).join(DATA_DIR_NAME).join(DICT_FILE_NAME);

        CeDownloader {
            base_path: base_path.to_string(),
            data_dir: data_dir.into_os_string().into_string().unwrap(),
            dict_file_path: dict_file_path.into_os_string().into_string().unwrap(),
        }
    }
    pub fn init_with_options(opts: &CeOptions) -> CeDownloader {
        let base_path = opts.base_path.clone();
        let data_dir = Path::new(&base_path).join(DATA_DIR_NAME);
        let dict_file_path = Path::new(&base_path).join(DATA_DIR_NAME).join(DICT_FILE_NAME);
        let data_dir_string = data_dir.into_os_string().into_string().unwrap();
        
        if !Path::new(&base_path).join(DATA_DIR_NAME).exists() {
            create_directory(&data_dir_string)
        }

        CeDownloader {
            base_path: base_path.to_string(),
            data_dir: (&data_dir_string).clone(),
            dict_file_path: dict_file_path.into_os_string().into_string().unwrap(),
        }
    }

    pub fn download(&self) -> Result<String, ureq::Error> {
        let resp = ureq::get(FILE_DOWNLOAD_LOCATION).call()?;
        let mut bytes: Vec<u8> = Vec::with_capacity(10_000_000);
        resp.into_reader()
            .take(10_000_000)
            .read_to_end(&mut bytes)?;
        let mut gz = GzDecoder::new(&bytes[..]);
        let mut s = String::new();
        gz.read_to_string(&mut s)?;
        write_file(&self.dict_file_path, &s);
        Ok(s)
    }
}
