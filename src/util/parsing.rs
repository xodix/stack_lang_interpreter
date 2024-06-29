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

    src.len() + 1
}

pub fn looks_like_number(src: &str) -> bool {
    let mut chars = src.chars();
    let ch1 = chars.next().unwrap_or(' ');
    let ch2 = chars.next().unwrap_or(' ');

    ch1.is_ascii_digit() || ch1 == '-' && ch2.is_ascii_digit()
}
