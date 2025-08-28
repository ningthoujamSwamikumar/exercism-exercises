use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasec = 10i64.pow(9u32); //returns i64, the type of base
    start.checked_add(Duration::new(gigasec, 0)).unwrap()
}
