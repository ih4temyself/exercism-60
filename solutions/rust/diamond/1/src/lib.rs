pub fn get_diamond(c: char) -> Vec<String> {
    let last = c as u8;
    let a = 'A' as u8;
    let size = (last - a) as usize;
    let width = size * 2 + 1;

    let mut rows = Vec::new();

    for i in 0..=size {
        let ch = (a + i as u8) as char;

        let core = if i == 0 {
            "A".to_string()
        } else {
            let mid = i * 2 - 1;
            format!("{}{}{}", ch, " ".repeat(mid), ch)
        };
        let padding = (width - core.len()) / 2;
        let line = format!("{}{}{}", " ".repeat(padding), core, " ".repeat(padding));
        rows.push(line);
    }

    for i in (0..size).rev() {
        rows.push(rows[i].clone());
    }
    rows
}