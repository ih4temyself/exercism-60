#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    seq: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    seq: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, ch) in dna.chars().enumerate() {
            match ch {
                'A' | 'C' | 'G' | 'T' => {}
                _ => return Err(i),
            }
        }
        Ok(Dna { seq: dna.to_string() })
    }

    pub fn into_rna(self) -> Rna {
        let mut out = String::new();
        for c in self.seq.chars() {
            let x = match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => '?',
            };
            out.push(x);
        }
        Rna { seq: out }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, ch) in rna.chars().enumerate() {
            match ch {
                'A' | 'C' | 'G' | 'U' => {}
                _ => return Err(i),
            }
        }
        Ok(Rna { seq: rna.to_string() })
    }
}