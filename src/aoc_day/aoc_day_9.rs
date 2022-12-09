use std::collections::HashSet;

use vector2d::Vector2D;

use crate::aoc_day;

pub struct DayRunner9 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner9 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let mut t_record: HashSet<(i32, i32)> = HashSet::new();
        let mut rope = vec![];
        make_rope(2, &mut rope);

        input.lines().for_each(|line| {
            let (direction, amount) = parse_line(line);

            for _ in 0..amount {
                simulate(&mut rope, direction);

                let last = rope.last().unwrap();
                t_record.insert((last.x, last.y));
            }
        });

        return format!("{}", t_record.len());
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let mut t_record: HashSet<(i32, i32)> = HashSet::new();
        let mut rope = vec![];
        make_rope(10, &mut rope);

        input.lines().for_each(|line| {
            let (direction, amount) = parse_line(line);

            for _ in 0..amount {
                simulate(&mut rope, direction);

                let last = rope.last().unwrap();
                t_record.insert((last.x, last.y));
            }
        });

        return format!("{}", t_record.len());
    }
}

fn parse_line(line: &str) -> (&str, i32) {
    let mut iter = line.split(" ");
    let direction = iter.next().expect("incorrect input.");
    let amount = iter
        .next()
        .expect("incorrect inputs.")
        .parse::<i32>()
        .expect("incorrect inputs,");

    return (direction, amount);
}

fn make_rope(knot_count: i32, rope: &mut Vec<Vector2D<i32>>) {
    for _ in 0..knot_count {
        rope.push(Vector2D::<i32> { x: 0, y: 0 })
    }
}

fn simulate(rope: &mut Vec<Vector2D<i32>>, direction: &str) {
    assert!(rope.len() > 0);
    let delta: &Vector2D<i32> = match direction {
        "R" => &Vector2D::<i32> { x: 1, y: 0 },
        "L" => &Vector2D::<i32> { x: -1, y: 0 },
        "U" => &Vector2D::<i32> { x: 0, y: 1 },
        "D" => &Vector2D::<i32> { x: 0, y: -1 },
        _ => panic!("incorrect inputs."),
    };

    rope[0] = rope[0] + *delta;
    for i in 1..rope.len() {
        rope[i] = resolve_tail(&rope[i - 1], &rope[i]);
    }
}

fn resolve_tail(h: &Vector2D<i32>, t: &Vector2D<i32>) -> Vector2D<i32> {
    let mut r_t = *t;
    let mut t_delta = h - t;

    if t_delta.x.abs() > 1 || t_delta.y.abs() > 1 {
        normalize(&mut t_delta);

        r_t = r_t + t_delta;
    }

    return r_t;
}

fn normalize(v: &mut Vector2D<i32>) {
    if v.x.abs() > 1 {
        v.x = v.x / v.x.abs();
    }

    if v.y.abs() > 1 {
        v.y = v.y / v.y.abs();
    }
}
