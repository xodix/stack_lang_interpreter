pub fn ignore_multiline(src: &str, i: &mut usize) {
    let comment_end = src.find("*/").unwrap_or(src.len());

    *i += comment_end + 3;
}

pub fn ignore_single_line(src: &str, i: &mut usize) {
    let comment_end = src.find('\n').unwrap_or(src.len());

    *i += comment_end;
}
