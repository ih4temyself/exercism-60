pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let n = size as usize;
    if n == 0 {
        return Vec::new();
    }

    let mut m = vec![vec![0; n]; n];
    let mut num = 1u32;

    let mut top = 0;
    let mut bottom = n as i32 - 1;
    let mut left = 0;
    let mut right = n as i32 - 1;

    while top <= bottom && left <= right {
        for c in left..=right {
            m[top as usize][c as usize] = num;
            num += 1;
        }
        top += 1;

        for r in top..=bottom {
            m[r as usize][right as usize] = num;
            num += 1;
        }
        right -= 1;

        if top <= bottom {
            for c in (left..=right).rev() {
                m[bottom as usize][c as usize] = num;
                num += 1;
            }
            bottom -= 1;
        }

        if left <= right {
            for r in (top..=bottom).rev() {
                m[r as usize][left as usize] = num;
                num += 1;
            }
            left += 1;
        }
    }

    m
}