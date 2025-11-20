pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

fn check_luhn(s: &str) -> bool {
    let mut digits = Vec::new();
    for ch in s.chars() {
        if ch == ' ' {
            continue;
        }
        if ch.is_ascii_digit() {
            digits.push(ch as u8 - b'0');
        } else {
            return false;
        }
    }
    if digits.len() <= 1 {
        return false;
    }
    let mut sum: u32 = 0;
    let mut double = false;
    for d in digits.iter().rev() {
        let mut v = *d as u32;
        if double {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
        }
        sum += v;
        double = !double;
    }
    sum % 10 == 0
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let s = self.to_string();
        check_luhn(&s)
    }
}