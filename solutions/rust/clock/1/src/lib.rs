use std::fmt;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Clock {
    mins: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut t = hours * 60 + minutes;
        let day = 24 * 60;
        while t < 0 {
            t += day;
        }
        while t >= day {
            t -= day;
        }
        Clock { mins: t }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.mins + minutes)
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h = self.mins / 60;
        let m = self.mins % 60;
        write!(f,"{:02}:{:02}",h,m)
    }
}