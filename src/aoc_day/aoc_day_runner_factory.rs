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
        2 => Some(Box::new(aoc_day::aoc_day_2::DayRunner2 {})),
        1 => Some(Box::new(aoc_day::aoc_day_1::DayRunner1 {})),
        _ => return None,
    };
}