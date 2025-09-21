use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration as Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.saturating_add(1e9.seconds())
    // unimplemented!("What time is a gigasecond later than {}", start);
}
