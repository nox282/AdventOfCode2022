use std::str::Lines;

use crate::aoc_day;

pub struct DayRunner10 {}

struct CPUState<'a> {
    pub instruction_buffer: &'a str,
    pub instruction_value: i32,
    pub instruction_cycles: i32,
    pub reg_x: i32,
}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner10 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let mut signal_strength = 0;
        let mut cpu_state = CPUState {
            instruction_buffer: "",
            instruction_value: 0,
            instruction_cycles: 0,
            reg_x: 1,
        };

        let mut input_iter = input.lines();
        for cycle in 1..221 {
            begin_cycle(&mut input_iter, &mut cpu_state);

            run_cycle(&mut cpu_state);

            if (cycle - 20) % 40 == 0 {
                signal_strength += cycle * cpu_state.reg_x;
            }

            end_cycle(&mut cpu_state);
        }

        return format!("{}", signal_strength);
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let mut cpu_state = CPUState {
            instruction_buffer: "",
            instruction_value: 0,
            instruction_cycles: 0,
            reg_x: 1,
        };

        let mut input_iter = input.lines();

        let mut display: String = "".to_string();
        for cycle in 1..241 {
            begin_cycle(&mut input_iter, &mut cpu_state);

            run_cycle(&mut cpu_state);

            let current_pixel = cycle % 40 - 1;
            if current_pixel == 0 {
                display += "\n";
            }

            let sprite_positions: Vec<i32> =
                vec![cpu_state.reg_x - 1, cpu_state.reg_x, cpu_state.reg_x + 1];
            if sprite_positions.contains(&current_pixel) {
                display += "#";
            } else {
                display += ".";
            }

            end_cycle(&mut cpu_state);
        }

        return format!("{}", display);
    }
}

fn begin_cycle(lines: &mut Lines, cpu_state: &mut CPUState) {
    if cpu_state.instruction_cycles > 0 {
        return;
    }

    match lines.next() {
        Some(line) => parse_line(line, cpu_state),
        None => {}
    }
}

fn run_cycle(cpu_state: &mut CPUState) {
    if cpu_state.instruction_cycles <= 0 {
        return;
    }

    cpu_state.instruction_cycles -= 1;
}

fn end_cycle(cpu_state: &mut CPUState) {
    if cpu_state.instruction_cycles <= 0 {
        cpu_state.reg_x = run_instruction(cpu_state);
        cpu_state.instruction_buffer = "";
    }
}

fn parse_line(line: &str, cpu_state: &mut CPUState) {
    let mut iter = line.split(" ");
    match iter.next() {
        Some("noop") => {
            cpu_state.instruction_buffer = "noop";
            cpu_state.instruction_cycles = 1;
            cpu_state.instruction_value = 0;
        }
        Some("addx") => {
            let v = iter
                .next()
                .expect("Incorrect inputs.")
                .parse::<i32>()
                .expect("Incorrect inputs");

            cpu_state.instruction_buffer = "addx";
            cpu_state.instruction_cycles = 2;
            cpu_state.instruction_value = v;
        }
        None | Some(_) => panic!("Incorrect inputs."),
    };
}

fn run_instruction(cpu_state: &CPUState) -> i32 {
    return match cpu_state.instruction_buffer {
        "noop" => cpu_state.reg_x,
        "addx" => cpu_state.reg_x + cpu_state.instruction_value,
        _ => panic!("incorrect inputs."),
    };
}
