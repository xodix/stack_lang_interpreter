pub mod cli;
pub mod error;
#[cfg(test)]
mod util_test;

use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn extract_src_text(path: PathBuf) -> String {
    let path = Path::new(&path);

    fs::read_to_string(path).expect("Could not read the file.\n")
}

pub fn extract_src_bin(path: PathBuf) -> Vec<u8> {
    let path = Path::new(&path);

    fs::read(path).expect("Could not read the file.\n")
}

pub fn write_file_bin(content: Vec<u8>, path: PathBuf) {
    fs::write(path, content).expect("Could not write to output file.");
}

/**
Macro that prints the amount of time that it takes to run the expression only if run in debug mode.

Macro returns the value of expression.
*/
#[macro_export]
macro_rules! log_debug_time {
    ( $function:expr, $what:expr ) => {
        if cfg!(debug_assertions) {
            use std::time::Instant;

            let now = Instant::now();
            let result = $function;
            println!("{} took {:?}", $what, now.elapsed());

            result
        } else {
            $function
        }
    };
}

/**
Function that gives the position of corresponding bracket in the string.
*/
pub fn find_closing_bracket(src: &str) -> usize {
    let mut open_brackets = 0;

    for (i, ch) in src.chars().enumerate() {
        if ch == '{' {
            open_brackets += 1;
        } else if ch == '}' {
            if open_brackets == 0 {
                return i + 1;
            } else {
                open_brackets -= 1;
            }
        }
    }

    src.len()
}
