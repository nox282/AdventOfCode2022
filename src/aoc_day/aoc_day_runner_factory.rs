use std::fmt;

use crate::aoc_day;

pub struct CreateDayFactoryError {
    e: String,
}

impl fmt::Display for CreateDayFactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "create day object: {}", self.e)
    }
}

pub fn create_day_runner(day: i32) -> Option<Box<dyn aoc_day::aoc_day::AOCDayRunner>> {
    #[allow(unreachable_code)]
    return match day {
        // _ADDADAY_
        _ => return None,
    };
}
