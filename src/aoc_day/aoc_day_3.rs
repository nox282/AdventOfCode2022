use std::collections::HashSet;

use crate::aoc_day;

static ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct DayRunner3 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner3 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let intersections: Vec<String> = input
            .lines()
            .map(|line| {
                // split lines
                let halves = line.split_at(line.len() / 2);
                let first: HashSet<char> = halves.0.chars().collect();
                let second: HashSet<char> = halves.1.chars().collect();

                //find intersection
                return first.intersection(&second).collect::<String>();
            })
            .collect();

        let total = intersections.iter().fold(0, |acc, i| {
            // for each intersections, accumulate the alphatecial value sum of all characters.
            return acc + i.chars().map(|c| get_alphabetical_value(&c)).sum::<usize>();
        });

        return format!("total priority: {}", total);
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        // TODO: this is shite.
        let lines: Vec<&str> = input.lines().collect();
        let mut i: usize = 0;
        let mut total: usize = 0;
        while i < lines.len() {
            // split lines into groups of 3.
            let group: Vec<&&str> = lines.split_at(i).1.iter().take(3).collect();
            if group.len() != 3 {
                panic!("incorrect inputs.");
            }

            // find the min length of all sets
            let min_len = group
                .iter()
                .map(|vec| vec.len())
                .min()
                .unwrap_or(usize::MAX);
            let min_index = group
                .iter()
                .position(|set| set.len() == min_len)
                .unwrap_or(usize::MAX);

            if min_len == usize::MAX {
                panic!("incorrect inputs.");
            }

            let mut common_char = HashSet::new();
            let reference_string: Vec<char> = group[min_index].chars().collect();
            for i in 0..reference_string.len() {
                let current_char = reference_string[i];

                if group[0].contains(current_char)
                    && group[1].contains(current_char)
                    && group[2].contains(current_char)
                {
                    common_char.insert(current_char);
                }
            }

            total += common_char
                .iter()
                .map(|c| get_alphabetical_value(c))
                .sum::<usize>();
            i += 3;
        }
        return format!("total priority: {}", total.to_string());
    }
}

fn get_alphabetical_value(t: &char) -> usize {
    return ALPHABET.iter().position(|c| c == t).unwrap_or_default() + 1;
}
