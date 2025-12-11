use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut res = HashMap::new();
    let chars: Vec<char> = words.chars().collect();
    let mut buf = String::new();
    for i in 0..chars.len() {
        let c = chars[i];
        if c.is_ascii_alphanumeric() {
            buf.push(c.to_ascii_lowercase());
        } else if c == '\'' {
            let left_ok = i > 0 && chars[i - 1].is_ascii_alphanumeric();
            let right_ok = i + 1 < chars.len() && chars[i + 1].is_ascii_alphanumeric();
            if left_ok && right_ok {
                buf.push('\'');
            } else if !buf.is_empty() {
                *res.entry(buf.clone()).or_insert(0) += 1;
                buf.clear();
            }
        } else if !buf.is_empty() {
            *res.entry(buf.clone()).or_insert(0) += 1;
            buf.clear();
        }
    }
    if !buf.is_empty() {
        *res.entry(buf).or_insert(0) += 1;
    }

    res
}
pub fn number(user_number: &str) -> Option<String> {
    let mut digits = String::new();
    for c in user_number.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
        } else if c.is_ascii_alphabetic() {
            return None;
        }
    }
    if digits.len() == 11 {
        if digits.starts_with('1') {
            digits.remove(0);
        } else {
            return None;
        }
    } else if digits.len() != 10 {
        return None;
    }
    let b = digits.as_bytes();
    if b[0] < b'2' || b[0] > b'9' {
        return None;
    }
    if b[3] < b'2' || b[3] > b'9' {
        return None;
    }
    Some(digits)
}