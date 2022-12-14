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
        13 => Some(Box::new(aoc_day::aoc_day_13::DayRunner13 {})),
        12 => Some(Box::new(aoc_day::aoc_day_12::DayRunner12 {})),
        11 => Some(Box::new(aoc_day::aoc_day_11::DayRunner11 {})),
        10 => Some(Box::new(aoc_day::aoc_day_10::DayRunner10 {})),
        9 => Some(Box::new(aoc_day::aoc_day_9::DayRunner9 {})),
        8 => Some(Box::new(aoc_day::aoc_day_8::DayRunner8 {})),
        7 => Some(Box::new(aoc_day::aoc_day_7::DayRunner7 {})),
        6 => Some(Box::new(aoc_day::aoc_day_6::DayRunner6 {})),
        5 => Some(Box::new(aoc_day::aoc_day_5::DayRunner5 {})),
        4 => Some(Box::new(aoc_day::aoc_day_4::DayRunner4 {})),
        3 => Some(Box::new(aoc_day::aoc_day_3::DayRunner3 {})),
        2 => Some(Box::new(aoc_day::aoc_day_2::DayRunner2 {})),
        1 => Some(Box::new(aoc_day::aoc_day_1::DayRunner1 {})),
        _ => return None,
    };
}