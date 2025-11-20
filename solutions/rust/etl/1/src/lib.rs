use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result_map = BTreeMap::new();

    for (score, letters) in h.iter() {
        for letter in letters {
            let tiny = letter.to_ascii_lowercase();
            result_map.insert(tiny, *score);
        }
    }
    result_map
}