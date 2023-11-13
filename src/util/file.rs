use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Binary {
    pub stack: Vec<crate::Stack>,
}

pub fn extract_text(path: &PathBuf) -> String {
    let path = Path::new(&path);

    fs::read_to_string(path).expect("Could not read the file.\n")
}

pub fn extract_bin(path: &PathBuf) -> Vec<u8> {
    let path = Path::new(&path);

    fs::read(path).expect("Could not read the file.\n")
}

pub fn write_bin(content: Vec<u8>, path: &PathBuf) {
    fs::write(path, content).expect("Could not write to output file.");
}
