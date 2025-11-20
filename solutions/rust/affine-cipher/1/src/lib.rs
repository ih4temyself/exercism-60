#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}
const M: i32 = 26;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % t;
        a = t;
    }
    a.abs()
}
fn mmi(a: i32) -> Option<i32> {
    for x in 0..M {
        let v = (a * x) % M;
        if v == 1 {
            return Some(x);
        }
    }
    None
}
fn norm_char(c: char) -> Option<char> {
    if c.is_ascii_alphabetic() {
        Some(c.to_ascii_lowercase())
    } else if c.is_ascii_digit() {
        Some(c)
    } else {
        None
    }
}
fn encode_letter(ch: char, a: i32, b: i32) -> char {
    if ch.is_ascii_digit() {
        return ch;
    }
    let i = (ch as u8 - b'a') as i32;
    let e = (a * i + b).rem_euclid(M);
    (b'a' + e as u8) as char
}
fn decode_letter(ch: char, ai: i32, b: i32) -> char {
    if ch.is_ascii_digit() {
        return ch;
    }
    let y = (ch as u8 - b'a') as i32;
    let d = ai * (y - b).rem_euclid(M);
    let r = d.rem_euclid(M);
    (b'a' + r as u8) as char
}
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, M) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mut raw = String::new();
    for c in plaintext.chars() {
        if let Some(x) = norm_char(c) {
            raw.push(x);
        }
    }

    let mut out = String::new();
    let mut cnt = 0;

    for ch in raw.chars() {
        let enc = encode_letter(ch, a, b);

        if cnt == 5 {
            out.push(' ');
            cnt = 0;
        }

        out.push(enc);
        cnt += 1;
    }

    Ok(out)
}
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, M) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let inv = match mmi(a) {
        Some(x) => x,
        None => return Err(AffineCipherError::NotCoprime(a)),
    };

    let mut out = String::new();

    for c in ciphertext.chars() {
        if !c.is_ascii_alphanumeric() {
            continue;
        }
        out.push(decode_letter(c, inv, b));
    }

    Ok(out)
}