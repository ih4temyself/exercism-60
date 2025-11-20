use std::cmp::{max, min};

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let h = garden.len();
    if h == 0 {
        return Vec::new();
    }
    let w = garden[0].len();
    let mut result = Vec::with_capacity(h);
    for y in 0..h {
        let row_bytes = garden[y].as_bytes();
        let mut new_row = String::with_capacity(w);
        for x in 0..w {
            if row_bytes[x] == b'*' {
                new_row.push('*');
                continue;
            }
            let mut count = 0;
            let y0 = if y == 0 { 0 } else { y - 1 };
            let y1 = min(y + 1, h - 1);
            let x0 = if x == 0 { 0 } else { x - 1 };
            let x1 = min(x + 1, w - 1);
            for ny in y0..=y1 {
                let bytes = garden[ny].as_bytes();
                for nx in x0..=x1 {
                    if ny == y && nx == x {
                        continue;
                    }
                    if bytes[nx] == b'*' {
                        count += 1;
                    }
                }
            }
            if count == 0 {
                new_row.push(' ');
            } else {
                new_row.push(char::from(b'0' + count));
            }
        }
        result.push(new_row);
    }

    result
}