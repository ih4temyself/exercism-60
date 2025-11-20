use std::sync::{Mutex, OnceLock};

static NAME_COUNTER: OnceLock<Mutex<u32>> = OnceLock::new();

fn counter_mutex() -> &'static Mutex<u32> {
    NAME_COUNTER.get_or_init(|| Mutex::new(0))
}

fn next_number() -> u32 {
    let mut guard = counter_mutex().lock().unwrap();
    let current = *guard;
    *guard += 1;
    current
}
fn number_to_name(n: u32) -> String {
    let mut s = String::with_capacity(5);

    let letters_index = n / 1000;
    let digits_index = n % 1000;

    let first_letter_index = (letters_index / 26) as u8;
    let second_letter_index = (letters_index % 26) as u8;

    s.push((b'A' + first_letter_index) as char);
    s.push((b'A' + second_letter_index) as char);

    let d1 = digits_index / 100;
    let d2 = (digits_index / 10) % 10;
    let d3 = digits_index % 10;

    s.push((b'0' + d1 as u8) as char);
    s.push((b'0' + d2 as u8) as char);
    s.push((b'0' + d3 as u8) as char);

    s
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let num = next_number();
        let name = number_to_name(num);
        Robot { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let num = next_number();
        self.name = number_to_name(num);
    }
}