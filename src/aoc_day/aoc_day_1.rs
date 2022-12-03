use crate::aoc_day;

pub struct DayRunner1 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner1 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let calories: Vec<i32> = break_down_inputs(input);
        let max_calories = calories.iter().max().unwrap_or(&i32::MIN);
        return format!("top calory is {}", max_calories.to_string());
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let mut calories: Vec<i32> = break_down_inputs(input);
        calories.sort_by(|a, b| b.cmp(a));
        let top_3_sum: i32 = calories.iter().take(3).sum();
        return format!("top 3 calory sum is {}", top_3_sum);
    }
}

fn break_down_inputs(input: &String) -> Vec<i32> {
    let mut calories: Vec<i32> = vec![];
    let chunks = input.split("\n\n");

    chunks.for_each(|chunk| {
        calories.push(
            chunk
                .lines()
                .map(|line| line.parse::<i32>().expect("Could not parse line"))
                .sum(),
        );
    });

    return calories;
}
