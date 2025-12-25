pub fn square_of_sum(n: u32) -> u32 {
    let mut total = 0u32;
    for i in 1..=n {
        total += i;
    }
    total * total
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut total = 0u32;
    for i in 1..=n {
        total += i * i;
    }
    total
}
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}