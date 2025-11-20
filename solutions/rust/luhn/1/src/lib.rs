pub fn is_valid(code: &str) -> bool {
    let studentish_clean: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    if studentish_clean.len() <= 1 {
        return false;
    }

    if !studentish_clean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let mut studentish_sum: u32 = 0;
    let mut studentish_double = false;

    for ch in studentish_clean.chars().rev() {
        let mut digit = match ch.to_digit(10) {
            Some(d) => d,
            None => return false,
        };

        if studentish_double {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }

        studentish_sum += digit;
        studentish_double = !studentish_double;
    }

    studentish_sum % 10 == 0
}