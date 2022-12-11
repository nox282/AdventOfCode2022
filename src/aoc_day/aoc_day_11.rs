use crate::aoc_day;

pub struct DayRunner11 {}

struct Monkey {
    items: Vec<i64>,
    operation: String,
    divisible_by: i64,
    target_id_if_true: usize,
    target_id_if_false: usize,
}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner11 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let mut monkeys = parse_monkeys(input);
        let mut inspected_count = vec![0; monkeys.len()];

        for _ in 0..20 {
            run_round(&mut monkeys, &mut inspected_count, |value| value / 3);
        }

        inspected_count.sort();
        inspected_count = inspected_count.into_iter().rev().collect();

        return format!("{}", inspected_count[0] * inspected_count[1]);
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let mut monkeys = parse_monkeys(input);
        let mut inspected_count = vec![0; monkeys.len()];

        let modulo_product = monkeys.iter().fold(1, |acc, m| acc * m.divisible_by);

        for _ in 0..10000 {
            run_round(&mut monkeys, &mut inspected_count, |value| {
                return value % modulo_product;
            });
        }

        inspected_count.sort();
        inspected_count = inspected_count.into_iter().rev().collect();

        return format!("{}", inspected_count[0] * inspected_count[1]);
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let lines: Vec<&str> = input.lines().collect();
    return lines
        .chunks(7)
        .map(|chunk| {
            let mut iter = chunk.iter();

            // Monkey 0:
            iter.next();

            //  Starting items: 54, 82, 90, 88, 86, 54
            let starting_items = iter
                .next()
                .expect("incorrect inputs")
                .replace("  Starting items: ", "")
                .split(", ")
                .map(|i| i.parse::<i64>().expect("incorrect inputs"))
                .collect();

            //  Operation: new = old * 7
            let operation = iter
                .next()
                .expect("incorrect inputs")
                .replace("  Operation: new = ", "");

            //  Test: divisible by 11
            let divisible_by = iter
                .next()
                .expect("incorrect inputs")
                .replace("  Test: divisible by ", "")
                .parse::<i64>()
                .expect("incorrect inputs");

            //    If true: throw to monkey 2
            let target_id_if_true = iter
                .next()
                .expect("incorrect inputs")
                .replace("    If true: throw to monkey ", "")
                .parse::<usize>()
                .expect("incorrect inputs");

            //    If false: throw to monkey 6
            let target_id_if_false = iter
                .next()
                .expect("incorrect inputs")
                .replace("    If false: throw to monkey ", "")
                .parse::<usize>()
                .expect("incorrect inputs");

            return Monkey {
                items: starting_items,
                operation: operation,
                divisible_by: divisible_by,
                target_id_if_true: target_id_if_true,
                target_id_if_false: target_id_if_false,
            };
        })
        .collect();
}

fn execute_operation(operation: &str, old_value: &i64) -> i64 {
    let mut iter = operation.split(" ");

    let lhs = match iter.next() {
        Some("old") => *old_value,
        Some(any_value) => any_value.parse::<i64>().expect("incorrect inputs"),
        None => panic!("incorrect inputs"),
    };

    let operator = iter.next().expect("incorrect inputs");

    let rhs = match iter.next() {
        Some("old") => *old_value,
        Some(any_value) => any_value.parse::<i64>().expect("incorrect inputs"),
        None => panic!("incorrect inputs"),
    };

    return match operator {
        "*" => lhs * rhs,
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "/" => lhs / rhs,
        _ => panic!("incorrect inputs"),
    };
}

fn get_throw_target_monkey_id(item_value: i64, monkey: &Monkey) -> usize {
    if item_value % monkey.divisible_by == 0 {
        return monkey.target_id_if_true;
    } else {
        return monkey.target_id_if_false;
    }
}

fn run_round(
    monkeys: &mut Vec<Monkey>,
    inspected_count: &mut Vec<i64>,
    evaluate: impl Fn(i64) -> i64,
) {
    for i in 0..monkeys.len() {
        for item_index in (0..monkeys[i].items.len()).rev() {
            inspected_count[i] += 1;

            let item_value = monkeys[i].items[item_index];
            let new_value = evaluate(execute_operation(
                monkeys[i].operation.as_str(),
                &item_value,
            ));

            let target_monkey_id = get_throw_target_monkey_id(new_value, &monkeys[i]);

            monkeys[i].items.remove(item_index);
            monkeys[target_monkey_id].items.push(new_value);
        }
    }
}
