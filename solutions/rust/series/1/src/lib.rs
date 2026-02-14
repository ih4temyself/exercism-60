pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec![];
    }
    if len > digits.len() {
        return Vec::new();
    }

    let mut res = Vec::new();
    let stop = digits.len() - len;

    for i in 0..=stop {
        let s = &digits[i..i+len];
        res.push(s.to_string());
    }

    res
}