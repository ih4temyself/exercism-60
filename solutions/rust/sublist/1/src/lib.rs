#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let studentish_a = first_list;
    let studentish_b = second_list;

    if studentish_a == studentish_b {
        return Comparison::Equal;
    }

    fn studentish_contains(big: &[i32], small: &[i32]) -> bool {
        if small.is_empty() {
            return true;
        }
        if small.len() > big.len() {
            return false;
        }

        for i in 0..=(big.len() - small.len()) {
            if big[i..i + small.len()] == *small {
                return true;
            }
        }
        false
    }

    if studentish_contains(studentish_a, studentish_b) {
        return Comparison::Superlist;
    }

    if studentish_contains(studentish_b, studentish_a) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}