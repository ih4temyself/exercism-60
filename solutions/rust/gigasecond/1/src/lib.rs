use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
use time::Duration;
pub fn after(start: DateTime) -> DateTime {
    let s = 1000000000;
    start + Duration::seconds(s)
}