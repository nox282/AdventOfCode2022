use std::collections::HashSet;

use crate::aoc_day;

pub struct DayRunner6 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner6 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        match find_first_sequence_length(input, 4) {
            usize::MAX => return String::from("incorrect input"),
            some_length => return format!("{}", some_length),
        }
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        match find_first_sequence_length(input, 14) {
            usize::MAX => return String::from("incorrect input"),
            some_length => return format!("{}", some_length),
        }
    }
}

fn find_first_sequence_length(input: &str, length: usize) -> usize {
    if input.len() < length {
        return usize::MAX;
    }

    for i in 0..input.len() - length {
        let sub_str = String::from(&input[i..i + length]);

        let mut set = HashSet::new();
        let mut iter = sub_str.chars();
        for _ in 0..sub_str.len() {
            if !set.insert(iter.next()) {
                break;
            }

            if set.len() == length {
                return i + length;
            }
        }
    }
    return usize::MAX;
}
