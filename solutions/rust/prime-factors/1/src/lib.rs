pub fn factors(mut n: u64) -> Vec<u64> {
    let mut list = Vec::new();

    if n < 2 {
        return list;
    }

    let mut div = 2u64;

    while div * div <= n {
        while n % div == 0 {
            list.push(div);
            n /= div;
        }
        div += 1;
    }
    if n > 1 {
        list.push(n);
    }

    list
}