use std::{cmp, fmt};

use crate::aoc_day;

struct Instruction {
    amount: i32,
    from_stack_index: usize,
    to_stack_index: usize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.amount, self.from_stack_index, self.to_stack_index
        )
    }
}

pub struct DayRunner5 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner5 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let parsed_inputs = parse_inputs(&input);
        let mut crates = parsed_inputs.0;
        let instructions = parsed_inputs.1;

        display_crates_with_desc(&crates, "crates before instructions:");

        instructions.iter().for_each(|instruction| {
            for _ in 0..instruction.amount {
                let le_crate = crates[instruction.from_stack_index].pop().unwrap();
                crates[instruction.to_stack_index].push(le_crate);
            }

            display_crates_with_desc(&crates, format!("{}: ", instruction).as_str());
        });

        display_crates_with_desc(&crates, "result:");
        return format!("{}", format_result_string(&crates));
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let parsed_inputs = parse_inputs(&input);
        let mut crates = parsed_inputs.0;
        let instructions = parsed_inputs.1;

        display_crates_with_desc(&crates, "crates before instructions:");

        instructions.iter().for_each(|instruction| {
            let mut les_crates: Vec<char> = vec![];
            for _ in 0..instruction.amount {
                les_crates.push(crates[instruction.from_stack_index].pop().unwrap());
            }

            les_crates.reverse();
            for le_crate in les_crates {
                crates[instruction.to_stack_index].push(le_crate);
            }

            display_crates_with_desc(&crates, format!("{}: ", instruction).as_str());
        });

        display_crates_with_desc(&crates, "result:");
        return format!("{}", format_result_string(&crates));
    }
}

fn parse_inputs(input: &String) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let splits: Vec<&str> = input.split("\r\n\r\n").collect();
    assert_eq!(splits.len(), 2);

    return (
        parse_input_crates(&splits[0].to_string()),
        splits[1]
            .lines()
            .map(|line| parse_instruction(line))
            .collect(),
    );
}

fn parse_input_crates(input: &String) -> Vec<Vec<char>> {
    let mut crates: Vec<Vec<char>> = vec![];

    // get stack count
    let lines: Vec<&str> = input.lines().collect();
    assert!(lines.len() > 0);
    let stack_count = (lines[0].len() + 1) / 4;

    // initialize stacks.
    for _ in 0..stack_count {
        crates.push(vec![]);
    }

    input.lines().for_each(|line| {
        let has_crates = line.contains("[");
        if !has_crates {
            return;
        }

        for i in 0..stack_count {
            let crate_index = i * 4;
            let crate_id = line.chars().nth(crate_index + 1).unwrap_or(' ');
            if crate_id == ' ' {
                continue;
            }

            crates[i].push(crate_id);
        }
    });

    crates.iter_mut().for_each(|stack| stack.reverse());
    return crates;
}

fn parse_instruction(line: &str) -> Instruction {
    let mut parsed_instructions = Instruction {
        amount: i32::MAX,
        from_stack_index: usize::MAX,
        to_stack_index: usize::MAX,
    };

    let mut iter = line.split(" ");

    let mut iter_value = Some("initial value");
    while iter_value != None {
        iter_value = iter.next();
        match iter_value {
            Some("move") => {
                parsed_instructions.amount = iter
                    .next()
                    .unwrap_or_default()
                    .parse::<i32>()
                    .expect("incorrect inputs")
            }
            Some("from") => {
                parsed_instructions.from_stack_index = iter
                    .next()
                    .unwrap_or_default()
                    .parse::<usize>()
                    .expect("incorrect inputs")
                    - 1
            }
            Some("to") => {
                parsed_instructions.to_stack_index = iter
                    .next()
                    .unwrap_or_default()
                    .parse::<usize>()
                    .expect("incorrect inputs")
                    - 1
            }
            None => continue,
            _ => panic!("incorrect inputs"),
        }
    }

    return parsed_instructions;
}

fn format_result_string(crates: &Vec<Vec<char>>) -> String {
    let mut result: String = String::from("");
    for stack in crates {
        result += stack.last().unwrap_or(&'?').to_string().as_str();
    }
    return result;
}

fn display_crates(crates: &Vec<Vec<char>>) {
    let tallest_stack = match crates.iter().map(|stack| stack.len()).max() {
        Some(tallest_stack) => tallest_stack,
        None => return,
    };

    // display crates
    for i in (1..tallest_stack + 1).rev() {
        crates.iter().for_each(|stack| {
            if stack.len() >= i && stack.len() > 0 {
                print!("[{}] ", stack[i - 1]);
            } else {
                print!("    ");
            }
        });

        print!("\n");
    }

    // display stack number identifiers
    for i in 0..crates.len() {
        print!(" {}  ", i + 1);
    }
    print!("\n");
}

fn display_crates_with_desc(crates: &Vec<Vec<char>>, description: &str) {
    let mut separator = String::from("");
    for _ in 0..(cmp::max(crates.len() * 4, description.len())) {
        separator += "-";
    }

    println!("{}", separator);
    println!("{}", description);
    display_crates(crates);
    println!("{}", separator);
}
