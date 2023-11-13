pub fn skip_multiline(src: &str, i: &mut usize, line_height: &mut usize, line_width: &mut usize) {
    let comment_end = src.find("*/").unwrap_or(src.len());

    let lines = src[..comment_end].matches('\n').count();
    if lines != 0 {
        *line_height += lines;
        *line_width = 0;
    }

    *i += comment_end + 2;
}

pub fn skip_singleline(src: &str, i: &mut usize) {
    let comment_end = src.find('\n').unwrap_or(src.len());

    *i += comment_end - 1;
}
