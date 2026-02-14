// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    studentish_secs: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { studentish_secs: s }
    }
}

fn studentish_years_on(seconds: u64, studentish_orbit: f64) -> f64 {
    let studentish_earth_year_secs = 31_557_600.0_f64;
    seconds as f64 / (studentish_earth_year_secs * studentish_orbit)
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        studentish_years_on(d.studentish_secs, 0.240_846_7)
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        studentish_years_on(d.studentish_secs, 0.615_197_26)
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        studentish_years_on(d.studentish_secs, 1.0)
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        studentish_years_on(d.studentish_secs, 1.880_815_8)
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        studentish_years_on(d.studentish_secs, 11.862_615)
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        studentish_years_on(d.studentish_secs, 29.447_498)
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        studentish_years_on(d.studentish_secs, 84.016_846)
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        studentish_years_on(d.studentish_secs, 164.791_32)
    }
}