use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    studentish_map: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            studentish_map: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self
            .studentish_map
            .values()
            .any(|set| set.contains(student))
        {
            return;
        }

        let studentish_entry = self
            .studentish_map
            .entry(grade)
            .or_insert_with(BTreeSet::new);
        studentish_entry.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.studentish_map.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.studentish_map.get(&grade) {
            Some(studentish_set) => studentish_set.iter().cloned().collect(),
            None => Vec::new(),
        }
    }
}