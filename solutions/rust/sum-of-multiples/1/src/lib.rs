pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut total = 0;

    'next_number: for value in 1..limit {
        for &factor in factors {
            if factor != 0 && value % factor == 0 {
                total += value;
                continue 'next_number;
            }
        }
    }
    total
}