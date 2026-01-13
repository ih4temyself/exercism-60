pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("square between 1 and 64");
    }

    let studentish = 2u64.pow((s - 1) as u32);
    studentish
}

pub fn total() -> u64 {
    let mut studentish = 0u64;
    let mut i = 1u32;
    while i <= 64 {
        studentish += square(i);
        i += 1;
    }
    studentish
}