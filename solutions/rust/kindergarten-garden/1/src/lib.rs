fn plant_word(ch: char) -> &'static str {
    match ch {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => ""
    }
}

pub fn plants(layout: &str, kid_name: &str) -> Vec<&'static str> {
    let kids_table = [
        "Alice","Bob","Charlie","David",
        "Eve","Fred","Ginny","Harriet",
        "Ileana","Joseph","Kincaid","Larry"
    ];

    let mut kid_pos = 0usize;
    for (idx , nm) in kids_table.iter().enumerate() {
        if *nm == kid_name {
            kid_pos = idx;
            break
        }
    }
    let mut rows = layout
        .lines()
        .filter(|ln| !ln.is_empty());
    let first_row  = rows.next().unwrap_or("");
    let second_row=rows.next().unwrap_or("");

    let start_idx = kid_pos * 2;
    let mut cup_plants:Vec<&'static str> = Vec::new();

    for ch1 in first_row
        .chars()
        .skip(start_idx)
        .take(2) {
        cup_plants.push(plant_word(ch1));
    }

    for ch2 in second_row
        .chars()
        .skip(start_idx)
        .take(2) {
        cup_plants.push(plant_word(ch2));
    }

    cup_plants
}