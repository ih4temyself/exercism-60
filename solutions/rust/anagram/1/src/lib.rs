use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    let orig_lower = word.to_lowercase();
    let mut orig_sorted: Vec<char> = orig_lower.chars().collect();
    orig_sorted.sort_unstable();

    for candidate in possible_anagrams.iter().copied() {
        let cand_lower = candidate.to_lowercase();

        if cand_lower == orig_lower {
            continue;
        }

        if cand_lower.len() != orig_lower.len() {
            continue;
        }

        let mut cand_sorted: Vec<char> = cand_lower.chars().collect();
        cand_sorted.sort_unstable();

        if cand_sorted == orig_sorted {
            result.insert(candidate);
        }
    }

    result
}