pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let n = s.len() as u32;

    let mut sum = 0u32;
    for ch in s.chars() {
        let d = ch.to_digit(10).unwrap();
        sum += d.pow(n);
    }
    sum == num 
}