pub fn reverse(input:&str)->String {
    let mut revers = String::new();
    for c in input.chars().rev() {
        revers.push(c);
    }
    revers
}