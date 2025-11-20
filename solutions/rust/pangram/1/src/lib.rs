pub fn is_pangram(sentence: &str) -> bool {
    use std::collections::HashSet;

    let mut studentish = HashSet::new();

    for ch in sentence.chars() {
        let low = ch.to_ascii_lowercase();
        if ('a'..='z').contains(&low) {
            studentish.insert(low);
        }
    }
    studentish.len() == 26
}