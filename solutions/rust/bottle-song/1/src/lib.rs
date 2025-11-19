fn num_word_big(n: u32) -> &'static str {
    match n {
        0 => "No",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => ""
    }
}
fn num_word_small(n: u32) -> &'static str {
    match n {
        0 => "no",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => ""
    }
}
fn bottle_word(n: u32) -> &'static str {
    if n == 1 { "bottle" } else { "bottles" }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses: Vec<String> = Vec::new();
    let mut n = start_bottles;

    for _ in 0..take_down {
        let line1 = format!(
            "{} green {} hanging on the wall,",
            num_word_big(n),
            bottle_word(n)
        );
        let line2 = format!(
            "{} green {} hanging on the wall,",
            num_word_big(n),
            bottle_word(n)
        );
        let line3 = "And if one green bottle should accidentally fall,".to_string();

        let next = if n == 0 { 0 } else { n - 1 };
        let line4 = format!(
            "There'll be {} green {} hanging on the wall.",
            num_word_small(next),
            bottle_word(next)
        );

        let verse = format!(
"{}
{}
{}
{}",
            line1, line2, line3, line4
        );

        verses.push(verse);

        if n > 0 {
            n -= 1;
        }
    }

    verses.join("\n\n")
}