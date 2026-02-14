pub fn check(candidate: &str) -> bool {
    let mut studentish_set = std::collections::HashSet::new();

    for ch in candidate.chars() {
        let low = ch.to_ascii_lowercase();

        if low.is_ascii_alphabetic() {
            if studentish_set.contains(&low) {
                return false;
            }
            studentish_set.insert(low);
        }
    }
    true
}