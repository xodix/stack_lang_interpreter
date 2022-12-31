pub fn skip_multiline(src: &str, i: &mut usize) {
    let comment_end = src.find("*/").unwrap_or(src.len());

    *i += comment_end + 2;
}

pub fn skip_singleline(src: &str, i: &mut usize) {
    let comment_end = src.find('\n').unwrap_or(src.len());

    *i += comment_end - 1;
}
