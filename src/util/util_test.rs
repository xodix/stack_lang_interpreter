use crate::{log_debug_time, util::find_closing_bracket};

#[test]
fn test_log_debug_macro() {
    fn expensive_operation() -> u8 {
        use std::thread::sleep;

        sleep(std::time::Duration::from_millis(3000));

        5
    }

    let result = log_debug_time!(expensive_operation(), "Expensive operation");

    assert_eq!(5, result);
}

#[test]
fn test_find_closing_bracket() {
    let src = "{1 2 3 {*} true if {*} false if}";

    assert_eq!(find_closing_bracket(&src[1..]), 31);

    println!("{}", &src[1..31]);
}
