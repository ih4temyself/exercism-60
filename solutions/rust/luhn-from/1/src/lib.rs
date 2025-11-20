pub struct Luhn {
    raw: String,
}
impl Luhn {
    pub fn is_valid(&self) -> bool {
        let s = self.raw.trim();
        let mut digits = Vec::new();
        for ch in s.chars() {
            if ch == ' ' {
                continue;
            }
            if ch.is_ascii_digit() {
                digits.push((ch as u8 - b'0') as u32);
            } else {
                return false;
            }
        }
        if digits.len() <= 1 {
            return false;
        }
        let mut sum = 0u32;
        let mut double = false;

        for d in digits.iter().rev() {
            let mut v = *d;
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
}
impl<T: ToString> From<T> for Luhn {
    fn from(value: T) -> Self {
        Luhn {
            raw: value.to_string(),
        }
    }
}