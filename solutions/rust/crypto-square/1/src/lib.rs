pub fn encrypt(input: &str) -> String {
    let mut norm = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
            norm.push(ch.to_ascii_lowercase());
        }
    }
    let n = norm.len();
    if n == 0 {
        return String::new();
    }

    let mut c = (n as f64).sqrt().ceil() as usize;
    let mut r = c;
    if c * (c - 1) >= n {
        r = c - 1;
    }

    let mut rows = Vec::new();
    let mut idx = 0;
    for _ in 0..r {
        let end = (idx + c).min(n);
        rows.push(norm[idx..end].to_string());
        idx += c;
    }
    let mut cols = Vec::new();
    for col in 0..c {
        let mut s = String::new();
        for row in 0..r {
            let line = &rows[row];
            if col < line.len() {
                s.push(line.chars().nth(col).unwrap());
            } else {
                s.push(' ');
            }
        }
        cols.push(s);
    }

    cols.join(" ")
}