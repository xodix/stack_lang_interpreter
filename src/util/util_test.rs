use crate::log_debug_time;

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
