#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let mut studentish_sum: u64 = 0;
    let mut d: u64 = 1;

    while d * d <= num {
        if num % d == 0 {
            let other = num / d;

            if d != num {
                studentish_sum += d;
            }
            if other != d && other != num {
                studentish_sum += other;
            }
        }
        d += 1;
    }
    let result = if studentish_sum == num {
        Classification::Perfect
    } else if studentish_sum > num {
        Classification::Abundant
    } else {
        Classification::Deficient
    };

    Some(result)
}