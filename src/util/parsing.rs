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

pub fn looks_like_digit(src: &str) -> bool {
    let ch = src.chars().next().unwrap();

    ch.is_digit(10) || (ch == '-' && src.chars().next().unwrap_or(' ').is_digit(10))
}
