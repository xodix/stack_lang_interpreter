#[cfg(test)]
mod util_test;

use std::{env::args, fs, path::Path};

#[inline]
fn extract_path() -> String {
    args()
        .nth(1)
        .expect("You need to supply a path to source code.\n\nStackLang.exe <PATH_TO_SRC>\n\n")
}

/**
Function that reads the source file by using the path given by the user as a cli argument.
*/
pub fn extract_src() -> String {
    let path = extract_path();
    let path = Path::new(&path);

    fs::read_to_string(path).expect("Could not read the file.\n")
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
