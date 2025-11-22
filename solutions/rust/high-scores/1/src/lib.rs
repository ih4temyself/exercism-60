#[derive(Debug)]
pub struct HighScores {
    vals: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            vals: scores.to_vec()
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.vals
    }

    pub fn latest(&self) -> Option<u32> {
        self.vals.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut best: Option<u32> = None;
        for &s in &self.vals {
            match best {
                None => best = Some(s),
                Some(b) if s > b => best = Some(s),
                _ => {}
            }
        }
        best
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.vals.clone();
        v.sort();
        v.reverse();
        if v.len() > 3 {
            v.truncate(3);
        }
        v
    }
}