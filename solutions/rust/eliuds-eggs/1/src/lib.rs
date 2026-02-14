pub fn egg_count(display_value: u32) -> usize {
    let mut val = display_value;
    let mut eggs = 0usize;
    while val > 0 {
        if val & 1 == 1 {
            eggs += 1;
        }
        val >>= 1;
    }

    eggs
}