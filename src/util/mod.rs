pub mod cli;
pub mod error;
pub mod file;
pub mod parsing;
#[cfg(test)]
mod util_test;

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
