pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut studentish_count = 0;

    for (a, b) in s1.bytes().zip(s2.bytes()) {
        if a != b {
            studentish_count += 1;
        }
    }
    Some(studentish_count)
}