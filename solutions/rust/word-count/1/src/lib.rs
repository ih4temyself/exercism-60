use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    let mut cur = String::new();
    let chars: Vec<char> = words.chars().collect();
    let n = chars.len();
    for i in 0..n {
        let c = chars[i];
        if c.is_ascii_alphanumeric() {
            cur.push(c.to_ascii_lowercase());
        } else if c == '\'' {
            let before = i > 0 && chars[i - 1].is_ascii_alphanumeric();
            let after = i + 1 < n && chars[i + 1].is_ascii_alphanumeric();
            if before && after {
                cur.push('\'');
            } else {
                if !cur.is_empty() {
                    *map.entry(cur.clone()).or_insert(0) += 1;
                    cur.clear();
                }
            }
        } else {
            if !cur.is_empty() {
                *map.entry(cur.clone()).or_insert(0) += 1;
                cur.clear();
            }
        }
    }
    if !cur.is_empty() {
        *map.entry(cur).or_insert(0) += 1;
    }
    map
}