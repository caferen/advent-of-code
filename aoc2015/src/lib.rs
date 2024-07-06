use std::{fs, path::Path};

pub fn input(path: &str) -> String {
    let path = Path::new(path);
    fs::read_to_string(path).expect("File could not be read")
}
