pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(pig_word)
        .collect::<Vec<_>>()
        .join(" ")
}
fn pig_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    let lower = word.to_ascii_lowercase();
    if starts_like_vowel(&lower) || lower.starts_with("xr") || lower.starts_with("yt") {
        return format!("{word}ay");
    }
    let split_at = find_split_index(&lower);

    let (head, tail) = word.split_at(split_at);
    format!("{tail}{head}ay")
}
fn starts_like_vowel(s: &str) -> bool {
    s.chars().next().map(is_vowel).unwrap_or(false)
}
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' |
                'A' | 'E' | 'I' | 'O' | 'U')
}
fn find_split_index(lower: &str) -> usize {
    let mut byte_index = 0;
    let mut prev_char: Option<char> = None;

    for ch in lower.chars() {
        let ch_len = ch.len_utf8();
        let at_start = byte_index == 0;

        let is_normal_vowel = matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u');
        let is_y_vowel_here = ch == 'y' && !at_start;

        if is_normal_vowel || is_y_vowel_here {
            if ch == 'u' {
                if let Some('q') = prev_char {
                    byte_index += ch_len; 
                }
            }
            break;
        }
        byte_index += ch_len;
        prev_char = Some(ch);
    }
    byte_index
}