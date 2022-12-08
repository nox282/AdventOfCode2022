use std::collections::HashMap;

use crate::aoc_day;

pub struct DayRunner7 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner7 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        const MAX_DIR_SIZE: u32 = 100_000;

        let mut dir_sizes: HashMap<String, u32> = HashMap::new();
        let mut path: Vec<String> = vec![];
        input
            .lines()
            .for_each(|line| parse_input(line, &mut path, &mut dir_sizes));

        let result = dir_sizes.iter().fold(0, |acc, kvp| {
            if kvp.1 <= &MAX_DIR_SIZE {
                return acc + kvp.1;
            }
            return acc;
        });
        return format!("{}", result);
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let mut dir_sizes: HashMap<String, u32> = HashMap::new();
        let mut path: Vec<String> = vec![];
        input
            .lines()
            .for_each(|line| parse_input(line, &mut path, &mut dir_sizes));

        let initial_size = dir_sizes["/"];
        println!("Initial size of root {}", initial_size);
        let free_space = 70000000 - initial_size;
        println!("Free space {}", free_space);
        let needed_space = 30000000 - free_space;
        println!("Needed space {}", needed_space);

        let result = dir_sizes.iter().fold(u32::MAX, |result, kvp| {
            if kvp.1 >= &needed_space && kvp.1 < &result {
                return *kvp.1;
            }
            return result;
        });

        return format!("{}", result);
    }
}

fn parse_input(line: &str, path: &mut Vec<String>, dir_size: &mut HashMap<String, u32>) {
    let mut splits = line.split(' ');
    match splits.next() {
        Some("$") => match splits.next() {
            Some("cd") => match splits.next() {
                Some("..") => {
                    path.pop();
                }

                Some(name) => {
                    path.push(name.to_string());
                }

                None => return,
            },

            Some("ls") => return,

            None | _ => return,
        },

        Some("dir") => return,

        Some(file_size_str) => {
            let file_size = file_size_str.parse::<u32>().expect("incorrect inputs");

            add_file(&build_path(path), &file_size, dir_size);
        }

        None => return,
    };
}

fn add_file(path: &str, file: &u32, dir_size: &mut HashMap<String, u32>) {
    let mut current_path = path.to_string();
    while current_path != "" {
        *dir_size.entry(current_path.to_string()).or_insert(0) += file;

        current_path.pop();

        let last_dir = match current_path.rsplit_once('/') {
            Some((_, dir)) => dir,
            None => return,
        };

        current_path = current_path[0..current_path.len() - last_dir.len()].to_string();
    }
}

fn build_path(path: &Vec<String>) -> String {
    if path.len() == 0 {
        return "".to_string();
    }

    let mut path_str = "".to_string();
    for dir in path {
        if dir == "/" {
            path_str = format!("{}", dir);
        } else {
            path_str = format!("{}{}/", path_str, dir);
        }
    }

    return path_str;
}
