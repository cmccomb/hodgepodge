//! Parse names and use enum values as map keys without optional features.
use hodgepodge::{Day, ParseEnumError};
use std::collections::HashMap;

fn main() -> Result<(), ParseEnumError> {
    let mut attendance: HashMap<Day, usize> = HashMap::new();
    for input in ["Monday", "monday", "Friday"] {
        let day: Day = input.parse()?;
        *attendance.entry(day).or_default() += 1;
    }
    // An explicit order makes the output deterministic despite HashMap iteration.
    for day in [Day::Monday, Day::Friday] {
        println!("{day}: {} visits", attendance[&day]);
    }
    Ok(())
}
