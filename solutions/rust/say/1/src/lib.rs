pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    fn small(num: u64) -> String {
        match num {
            0 => "".to_string(),
            1 => "one".into(),
            2 => "two".into(),
            3 => "three".into(),
            4 => "four".into(),
            5 => "five".into(),
            6 => "six".into(),
            7 => "seven".into(),
            8 => "eight".into(),
            9 => "nine".into(),
            10 => "ten".into(),
            11 => "eleven".into(),
            12 => "twelve".into(),
            13 => "thirteen".into(),
            14 => "fourteen".into(),
            15 => "fifteen".into(),
            16 => "sixteen".into(),
            17 => "seventeen".into(),
            18 => "eighteen".into(),
            19 => "nineteen".into(),
            _ => unreachable!(),
        }
    }
    fn tens(num: u64) -> String {
        match num {
            20 => "twenty".into(),
            30 => "thirty".into(),
            40 => "forty".into(),
            50 => "fifty".into(),
            60 => "sixty".into(),
            70 => "seventy".into(),
            80 => "eighty".into(),
            90 => "ninety".into(),
            _ => unreachable!(),
        }
    }
    fn to_999(mut x: u64) -> String {
        let mut out = String::new();

        if x >= 100 {
            let h = x / 100;
            out.push_str(&small(h));
            out.push_str(" hundred");
            x %= 100;
            if x > 0 {
                out.push(' ');
            }
        }
        if x >= 20 {
            let t = (x / 10) * 10;
            out.push_str(&tens(t));
            let r = x % 10;
            if r > 0 {
                out.push('-');
                out.push_str(&small(r));
            }
        } else if x > 0 {
            out.push_str(&small(x));
        }
        out
    }

    let units = [
        (1_000_000_000_000_000_000, "quintillion"),
        (1_000_000_000_000_000, "quadrillion"),
        (1_000_000_000_000, "trillion"),
        (1_000_000_000, "billion"),
        (1_000_000, "million"),
        (1000, "thousand"),
        (1, ""),
    ];
    let mut num = n;
    let mut parts = Vec::new();
    for (val, name) in units {
        if num >= val {
            let g = num / val;
            num %= val;
            let chunk = to_999(g);
            if name.is_empty() {
                parts.push(chunk);
            } else {
                parts.push(format!("{} {}", chunk, name));
            }
        }
    }
    parts.join(" ")
}