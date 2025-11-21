pub struct Allergies {
    studentish_score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,     
    Shellfish,   
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen, 
    Cats,        
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { studentish_score: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let studentish_val = match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        };

        (self.studentish_score & studentish_val) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut studentish_vec = Vec::new();

        let all = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        for a in all.iter() {
            if self.is_allergic_to(a) {
                studentish_vec.push(a.clone());
            }
        }

        studentish_vec
    }
}