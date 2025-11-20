pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return Vec::new();
    }
    let n = upper_bound as usize;
    let mut mark = vec![false; n + 1]; 
    let mut out = Vec::new();
    for i in 2..=n {
        if !mark[i] {
            out.push(i as u64);

            let mut m = i * 2;
            while m <= n {
                mark[m] = true;
                m += i;
            }
        }
    }
    out
}