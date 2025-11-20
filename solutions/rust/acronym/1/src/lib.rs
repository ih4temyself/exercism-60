pub fn abbreviate(phrase: &str) -> String {
    let mut studentish_acro = String::new();
    let mut studentish_at_word_start = true;
    let mut studentish_prev: Option<char> = None;

    for ch in phrase.chars() {
        if ch.is_alphabetic() {
            let camel_boundary = match studentish_prev {
                Some(p) => p.is_lowercase() && ch.is_uppercase(),
                None => false,
            };

            if studentish_at_word_start || camel_boundary {
                studentish_acro.push(ch.to_ascii_uppercase());
            }

            studentish_at_word_start = false;
        } else if ch == ' ' || ch == '-' || ch == '_' {
            studentish_at_word_start = true;
        } else {
            studentish_at_word_start = false;
        }

        studentish_prev = Some(ch);
    }

    studentish_acro
}