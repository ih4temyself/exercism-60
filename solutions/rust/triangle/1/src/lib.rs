pub struct Triangle {
    studentish_sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut studentish = sides;

        studentish.sort();

        if studentish[0] == 0 {
            return None;
        }

        if let Some(studentish_sum) = studentish[0].checked_add(studentish[1]) {
            if studentish_sum <= studentish[2] {
                return None;
            }
        } else {
            return None;
        }

        Some(Triangle {
            studentish_sides: studentish,
        })
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, b, c] = self.studentish_sides;
        a == b && b == c
    }

    pub fn is_scalene(&self) -> bool {
        let [a, b, c] = self.studentish_sides;
        a != b && b != c && a != c
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.studentish_sides;
        a == b || b == c || a == c
    }
}