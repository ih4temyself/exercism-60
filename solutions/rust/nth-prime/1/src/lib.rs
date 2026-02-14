pub fn nth(n: u32) -> u32 {
    let mut index = 0u32;
    let mut candidate = 2u32;

    loop {
        if is_prime(candidate) {
            if index == n {
                return candidate;
            }
            index += 1;
        }
        candidate += 1;
    }
}

fn is_prime(value: u32) -> bool {
    if value < 2 {
        return false;
    }
    if value == 2 {
        return true;
    }
    if value % 2 == 0 {
        return false;
    }

    let mut divisor = 3u32;
    while divisor * divisor <= value {
        if value % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}