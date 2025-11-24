pub fn answer(command: &str) -> Option<i32> {
    let mut studentish = command.trim();

    if !studentish.starts_with("What is ") || !studentish.ends_with('?') {
        return None;
    }

    studentish = &studentish[8..studentish.len().saturating_sub(1)];

    if studentish.is_empty() {
        return None;
    }

    let parts: Vec<&str> = studentish.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    fn studentish_number(txt: &str) -> Option<i32> {
        txt.parse::<i32>().ok()
    }

    let mut idx = 0usize;
    let first = studentish_number(parts[idx])?;
    idx += 1;

    let mut result_studentish = first;

    while idx < parts.len() {
        let op = parts[idx];

        match op {
            "plus" => {
                idx += 1;
                if idx >= parts.len() {
                    return None;
                }
                let rhs = studentish_number(parts[idx])?;
                result_studentish = result_studentish.checked_add(rhs)?;
                idx += 1;
            }
            "minus" => {
                idx += 1;
                if idx >= parts.len() {
                    return None;
                }
                let rhs = studentish_number(parts[idx])?;
                result_studentish = result_studentish.checked_sub(rhs)?;
                idx += 1;
            }
            "multiplied" => {
                idx += 1;
                if idx >= parts.len() || parts[idx] != "by" {
                    return None;
                }
                idx += 1;
                if idx >= parts.len() {
                    return None;
                }
                let rhs = studentish_number(parts[idx])?;
                result_studentish = result_studentish.checked_mul(rhs)?;
                idx += 1;
            }
            "divided" => {
                idx += 1;
                if idx >= parts.len() || parts[idx] != "by" {
                    return None;
                }
                idx += 1;
                if idx >= parts.len() {
                    return None;
                }
                let rhs = studentish_number(parts[idx])?;
                if rhs == 0 {
                    return None;
                }
                result_studentish = result_studentish.checked_div(rhs)?;
                idx += 1;
            }
            _ => {
                return None;
            }
        }
    }

    Some(result_studentish)
}