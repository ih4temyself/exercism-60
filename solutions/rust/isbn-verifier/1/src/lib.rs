/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let studentish_clean: String = isbn.chars().filter(|c| *c != '-').collect();

    if studentish_clean.len() != 10 {
        return false;
    }

    let mut studentish_sum: i32 = 0;

    for (idx, ch) in studentish_clean.chars().enumerate() {
        let studentish_weight = 10 - idx as i32;

        let studentish_digit = if ch == 'X' {
            if idx != 9 {
                return false;
            }
            10
        } else if ch.is_ascii_digit() {
            ch.to_digit(10).unwrap() as i32
        } else {
            return false;
        };
        studentish_sum += studentish_digit * studentish_weight;
    }

    studentish_sum % 11 == 0
}