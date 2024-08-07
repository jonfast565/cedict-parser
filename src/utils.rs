use std::fs;

pub struct CeOptions {
    pub base_path: String
}

pub fn write_file(file_path: &String, data: &String) {
    fs::write(file_path, data).expect("Unable to write file");
}

pub fn create_directory(dir_path: &String) {
    fs::create_dir(dir_path).expect("Unable to create directory");
}