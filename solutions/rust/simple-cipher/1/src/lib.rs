use rand::{thread_rng, Rng};

pub fn encode(key: &str, plain: &str) -> Option<String> {
    apply_cipher(key, plain, Direction::Forward)
}

pub fn decode(key: &str, cipher: &str) -> Option<String> {
    apply_cipher(key, cipher, Direction::Backward)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = generate_random_key(100);
    let encoded = encode(&key, s).unwrap_or_default();
    (key, encoded)
}

#[derive(Copy, Clone)]
enum Direction {
    Forward,
    Backward,
}
fn apply_cipher(key: &str, text: &str, direction: Direction) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }

    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    let mut result = String::with_capacity(text.len());

    for (idx, ch) in text.chars().enumerate() {
        if ch.is_ascii_lowercase() {
            let shift = (key_bytes[idx % key_len] - b'a') as i8;
            let shifted = match direction {
                Direction::Forward => rotate_char(ch, shift),
                Direction::Backward => rotate_char(ch, -shift),
            };
            result.push(shifted);
        } else {
            result.push(ch);
        }
    }

    Some(result)
}

fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.bytes().all(|b| (b'a'..=b'z').contains(&b))
}
fn rotate_char(c: char, shift: i8) -> char {
    let base = b'a';
    let offset = (c as u8) - base;
    let rotated = (offset as i8 + shift).rem_euclid(26) as u8;
    (base + rotated) as char
}
fn generate_random_key(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len)
        .map(|_| {
            let n: u8 = rng.gen_range(0..26);
            (b'a' + n) as char
        })
        .collect()
}