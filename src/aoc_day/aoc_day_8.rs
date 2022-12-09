use crate::aoc_day;

pub struct DayRunner8 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner8 {
    fn run_part_1(&self, input: &String, test_input: &String) -> String {
        let mut grid: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                return line
                    .chars()
                    .map(|c| c.to_string().parse::<u8>().expect("Incorrect inputs"))
                    .collect();
            })
            .collect();
        let height = grid.len();
        let width = grid[0].len();

        let mut visible_count = 0;
        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == height - 1 {
                    visible_count += 1;
                    continue;
                }

                if y == 0 || y == width - 1 {
                    visible_count += 1;
                    continue;
                }

                if is_visible(&(x, y), &grid, height, width) {
                    visible_count += 1;
                }
            }
        }

        return format!("{}", visible_count);
    }

    fn run_part_2(&self, input: &String, test_input: &String) -> String {
        let mut grid: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                return line
                    .chars()
                    .map(|c| c.to_string().parse::<u8>().expect("Incorrect inputs"))
                    .collect();
            })
            .collect();

        grid.iter();

        let height = grid.len();
        let width = grid[0].len();

        let mut scenic_score: Vec<i32> = vec![];
        for y in 0..height {
            for x in 0..width {
                scenic_score.push(get_scenic_score(&(x, y), &grid, height, width));
            }
        }

        return format!("{}", scenic_score.iter().max().unwrap());
    }
}

fn is_visible(pos: &(usize, usize), grid: &Vec<Vec<u8>>, height: usize, width: usize) -> bool {
    let x = pos.0;
    let y = pos.1;

    // left
    for xd in (0..x).rev() {
        if grid[xd][y] >= grid[x][y] {
            break;
        }

        if xd == 0 {
            return true;
        }
    }

    // right
    for xd in x + 1..height {
        if grid[xd][y] >= grid[x][y] {
            break;
        }

        if xd == height - 1 {
            return true;
        }
    }

    // up
    for yd in (0..y).rev() {
        if grid[x][yd] >= grid[x][y] {
            break;
        }

        if yd == 0 {
            return true;
        }
    }

    // down
    for yd in y + 1..width {
        if grid[x][yd] >= grid[x][y] {
            break;
        }

        if yd == width - 1 {
            return true;
        }
    }

    return false;
}

fn get_scenic_score(pos: &(usize, usize), grid: &Vec<Vec<u8>>, height: usize, width: usize) -> i32 {
    let x = pos.0;
    let y = pos.1;

    let mut result = 1;
    let mut counter = 0;

    // left
    for xd in (0..x).rev() {
        counter += 1;
        if grid[xd][y] >= grid[x][y] {
            break;
        }
    }

    result *= counter;
    counter = 0;

    // right
    for xd in x + 1..height {
        counter += 1;
        if grid[xd][y] >= grid[x][y] {
            break;
        }
    }

    result *= counter;
    counter = 0;

    // up
    for yd in (0..y).rev() {
        counter += 1;
        if grid[x][yd] >= grid[x][y] {
            break;
        }
    }

    result *= counter;
    counter = 0;

    // down
    for yd in y + 1..width {
        counter += 1;
        if grid[x][yd] >= grid[x][y] {
            break;
        }
    }

    result *= counter;
    counter = 0;

    return result;
}
