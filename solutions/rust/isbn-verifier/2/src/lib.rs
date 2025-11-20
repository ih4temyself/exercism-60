/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let clean: String = isbn.chars().filter(|c| *c != '-').collect();

    if clean.len() != 10 {
        return false;
    }

    let mut sum: i32 = 0;

    for (idx, ch) in clean.chars().enumerate() {
        let weight = 10 - idx as i32;

        let digit = if ch == 'X' {
            if idx != 9 {
                return false;
            }
            10
        } else if ch.is_ascii_digit() {
            ch.to_digit(10).unwrap() as i32
        } else {
            return false;
        };
        sum += digit * weight;
    }

    sum % 11 == 0
}