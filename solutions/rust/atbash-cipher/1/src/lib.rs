pub fn encode(plain: &str) -> String {
    fn atbash(c: char) -> char {
        if c.is_ascii_alphabetic() {
            let x = c.to_ascii_lowercase() as u8 - b'a';
            (b'z' - x) as char
        } else {
            c
        }
    }
    let mut out = String::new();
    let mut cnt = 0;
    for ch in plain.chars() {
        if ch.is_ascii_alphanumeric() {
            let mut c = ch;
            if c.is_ascii_alphabetic() {
                c = atbash(c);
            } else {
                c = c;
            }
            if cnt == 5 {
                out.push(' ');
                cnt = 0;
            }
            out.push(c.to_ascii_lowercase());
            cnt += 1;
        }
    }

    out
}
pub fn decode(cipher: &str) -> String {
    fn atbash(c: char) -> char {
        if c.is_ascii_alphabetic() {
            let x = c.to_ascii_lowercase() as u8 - b'a';
            (b'z' - x) as char
        } else {
            c
        }
    }
    let mut out = String::new();
    for ch in cipher.chars() {
        if ch == ' ' {
            continue;
        }
        if ch.is_ascii_digit() {
            out.push(ch);
        } else if ch.is_ascii_alphabetic() {
            out.push(atbash(ch));
        }
    }
    out
}