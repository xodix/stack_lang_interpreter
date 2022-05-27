use std::{env::args, fs, path::Path};

fn extract_path() -> String {
    args()
        .nth(1)
        .expect("You need to supply the path to source code.\n\nStackLang.exe <PATH_TO_SRC>\n\n")
}

/**
Function that reads the source file by using the path given by the user.
*/
pub fn extract_src() -> String {
    let path = extract_path();
    let path = Path::new(&path);

    fs::read_to_string(path).expect("Could not read the file.\n")
}
