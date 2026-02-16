use unicode_segmentation::UnicodeSegmentation;

const SPECIAL_CHARS: [&'static str; 9] = ["/", ":", "*", "?", "\"", "<", ">", "|", "\\"];

/// 清理文件名中的非法字符
pub fn sanitize_filename(filename: &str) -> String {
    UnicodeSegmentation::graphemes(filename, true)
        .map(|g| if !SPECIAL_CHARS.contains(&g) { g } else { "" })
        .collect::<String>()
        .trim()
        .to_string()
}

/// 截断文件名，保留头部和尾部，中间用 "..." 连接
pub fn truncate_filename(filename: &str, max_length: usize) -> String {
    let filename = sanitize_filename(filename);

    let graphemes: Vec<&str> = UnicodeSegmentation::graphemes(filename.as_str(), true).collect();
    let total_len = graphemes.len();

    if total_len <= max_length {
        return filename;
    }

    let ellipsis = "...";
    let ellipsis_len = UnicodeSegmentation::graphemes(ellipsis, true).count();
    let available_len = max_length.saturating_sub(ellipsis_len);

    if available_len == 0 {
        return ellipsis.to_string();
    }

    let head_len = available_len / 2;
    let tail_len = available_len - head_len;

    let head: String = graphemes[..head_len].concat();
    let tail: String = graphemes[total_len.saturating_sub(tail_len)..].concat();

    format!("{}{}{}", head, ellipsis, tail)
}
