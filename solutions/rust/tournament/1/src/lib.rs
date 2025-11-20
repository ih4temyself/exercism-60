use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    #[derive(Default, Clone)]
    struct StudentishStats {
        mp: u32,
        w: u32,
        d: u32,
        l: u32,
        p: u32,
    }

    let mut studentish_map: HashMap<String, StudentishStats> = HashMap::new();

    for studentish_line in match_results.lines() {
        let studentish_line = studentish_line.trim();
        if studentish_line.is_empty() {
            continue;
        }

        let studentish_parts: Vec<&str> = studentish_line.split(';').collect();
        if studentish_parts.len() != 3 {
            continue;
        }

        let studentish_team1 = studentish_parts[0].to_string();
        let studentish_team2 = studentish_parts[1].to_string();
        let studentish_result = studentish_parts[2];

        match studentish_result {
            "win" => {
                {
                    let studentish_entry1 = studentish_map
                        .entry(studentish_team1.clone())
                        .or_insert_with(StudentishStats::default);
                    studentish_entry1.mp += 1;
                    studentish_entry1.w += 1;
                    studentish_entry1.p += 3;
                }
                {
                    let studentish_entry2 = studentish_map
                        .entry(studentish_team2.clone())
                        .or_insert_with(StudentishStats::default);
                    studentish_entry2.mp += 1;
                    studentish_entry2.l += 1;
                }
            }
            "loss" => {
                {
                    let studentish_entry1 = studentish_map
                        .entry(studentish_team1.clone())
                        .or_insert_with(StudentishStats::default);
                    studentish_entry1.mp += 1;
                    studentish_entry1.l += 1;
                }
                {
                    let studentish_entry2 = studentish_map
                        .entry(studentish_team2.clone())
                        .or_insert_with(StudentishStats::default);
                    studentish_entry2.mp += 1;
                    studentish_entry2.w += 1;
                    studentish_entry2.p += 3;
                }
            }
            "draw" => {
                {
                    let studentish_entry1 = studentish_map
                        .entry(studentish_team1.clone())
                        .or_insert_with(StudentishStats::default);
                    studentish_entry1.mp += 1;
                    studentish_entry1.d += 1;
                    studentish_entry1.p += 1;
                }
                {
                    let studentish_entry2 = studentish_map
                        .entry(studentish_team2.clone())
                        .or_insert_with(StudentishStats::default);
                    studentish_entry2.mp += 1;
                    studentish_entry2.d += 1;
                    studentish_entry2.p += 1;
                }
            }
            _ => {}
        }
    }

    let mut studentish_rows: Vec<(String, StudentishStats)> =
        studentish_map.into_iter().collect();

    studentish_rows.sort_by(|(name_a, stats_a), (name_b, stats_b)| {
        if stats_a.p != stats_b.p {
            stats_b.p.cmp(&stats_a.p)
        } else {
            name_a.cmp(name_b)
        }
    });

    let mut studentish_out = String::new();
    studentish_out.push_str("Team                           | MP |  W |  D |  L |  P");

    for (studentish_name, studentish_stats) in studentish_rows {
        let studentish_line = format!(
            "\n{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            studentish_name,
            studentish_stats.mp,
            studentish_stats.w,
            studentish_stats.d,
            studentish_stats.l,
            studentish_stats.p
        );
        studentish_out.push_str(&studentish_line);
    }

    studentish_out
}