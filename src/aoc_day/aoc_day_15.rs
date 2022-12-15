use std::cmp;

use vector2d::Vector2D;

type Rect = (i64, i64, i64, i64);
type Sensor = (Vector2D<i64>, Vector2D<i64>);

use crate::aoc_day;

pub struct DayRunner15 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner15 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let sensors = parse_input(input);
        let world_bounds = calculate_world_bounds(&sensors);
        let ans = find_points_that_cannot_contain_a_beacon(
            &sensors,
            (world_bounds.0, 2000000, world_bounds.2, 2000000),
        )
        .len();
        return format!("{}", ans);
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let sensors = parse_input(&input);
        let borders: Vec<Vec<Vector2D<i64>>> = sensors
            .iter()
            .enumerate()
            .map(|s| get_border_points(&s.1))
            .collect();

        for border in borders.iter() {
            for point in border.iter() {
                if point.x < 0 || point.y < 0 || point.x > 4000000 || point.y > 4000000 {
                    continue;
                }

                if !in_range_of_atleast_one(point, &sensors) {
                    return format!("{}", point.x * 4000000 + point.y);
                }
            }
        }
        return format!("no answer");
    }
}

fn in_range_of_atleast_one(p: &Vector2D<i64>, sensors: &Vec<Sensor>) -> bool {
    for sensor in sensors.iter() {
        if is_in_range(p, sensor) {
            return true;
        }
    }
    return false;
}

fn get_border_points(sensor: &Sensor) -> Vec<Vector2D<i64>> {
    let mut points: Vec<Vector2D<i64>> = vec![];
    let distance = manhattan_distance(&sensor.0, &sensor.1);

    let up = Vector2D::<i64> {
        x: sensor.0.x,
        y: sensor.0.y + distance + 1,
    };

    let right = Vector2D::<i64> {
        x: sensor.0.x + distance + 1,
        y: sensor.0.y,
    };

    let down = Vector2D::<i64> {
        x: sensor.0.x,
        y: sensor.0.y - (distance + 1),
    };

    let left = Vector2D::<i64> {
        x: sensor.0.x - (distance + 1),
        y: sensor.0.y,
    };

    let mut current = up;
    let mut step = right - up;
    normalize(&mut step);
    while current != right {
        points.push(current);
        current += step;
    }

    current = right;
    let mut step = down - right;
    normalize(&mut step);
    while current != down {
        points.push(current);
        current += step;
    }

    current = down;
    step = left - down;
    normalize(&mut step);
    while current != left {
        points.push(current);
        current += step;
    }

    current = left;
    let mut step = up - left;
    normalize(&mut step);
    while current != up {
        points.push(current);
        current += step;
    }

    return points;
}

fn parse_input(input: &str) -> Vec<Sensor> {
    return input
        .lines()
        .map(|line| {
            // Sensor at x={X}, y={Y}: closest beacon is at x={X2}, y={Y2}
            let cleaned_up_line = line
                .replace("Sensor at x=", "")
                .replace(" y=", "")
                .replace(": closest beacon is at ", ",")
                .replace("x=", "");

            let splits: Vec<i64> = cleaned_up_line
                .split(",")
                .map(|s| s.parse::<i64>().expect("incorrect inputs."))
                .collect();
            assert_eq!(splits.len(), 4);

            return (
                Vector2D::<i64> {
                    x: splits[0],
                    y: splits[1],
                },
                Vector2D::<i64> {
                    x: splits[2],
                    y: splits[3],
                },
            );
        })
        .collect();
}

fn find_points_that_cannot_contain_a_beacon(
    sensors: &Vec<Sensor>,
    search_area: Rect,
) -> Vec<Vector2D<i64>> {
    let mut ans = vec![];
    for x in search_area.0..search_area.2 + 1 {
        for y in search_area.1..search_area.3 + 1 {
            let p = Vector2D::<i64> { x, y };

            for sensor in sensors.iter() {
                if is_in_range(&p, sensor) && p != sensor.0 && p != sensor.1 {
                    ans.push(p);
                    break;
                }
            }
        }
    }

    return ans;
}

fn is_in_range(p: &Vector2D<i64>, sensor: &Sensor) -> bool {
    let beacon_distance = manhattan_distance(&sensor.0, &sensor.1);
    let p_distance = manhattan_distance(&sensor.0, &p);

    return p_distance <= beacon_distance;
}

fn manhattan_distance(a: &Vector2D<i64>, b: &Vector2D<i64>) -> i64 {
    return (b.x - a.x).abs() + (b.y - a.y).abs();
}

fn calculate_world_bounds(sensors: &Vec<(Vector2D<i64>, Vector2D<i64>)>) -> Rect {
    let sensor_bounds = (
        sensors
            .iter()
            .map(|s| s.0.x)
            .min()
            .expect("incorrect inputs"),
        sensors
            .iter()
            .map(|s| s.0.y)
            .min()
            .expect("incorrect inputs"),
        sensors
            .iter()
            .map(|s| s.0.x)
            .max()
            .expect("incorrect inputs"),
        sensors
            .iter()
            .map(|s| s.0.y)
            .max()
            .expect("incorrect inputs"),
    );

    let beacon_bounds = (
        sensors
            .iter()
            .map(|s| s.1.x)
            .min()
            .expect("incorrect inputs"),
        sensors
            .iter()
            .map(|s| s.1.y)
            .min()
            .expect("incorrect inputs"),
        sensors
            .iter()
            .map(|s| s.1.x)
            .max()
            .expect("incorrect inputs"),
        sensors
            .iter()
            .map(|s| s.1.y)
            .max()
            .expect("incorrect inputs"),
    );

    let largest_distance = sensors
        .iter()
        .map(|s| manhattan_distance(&s.0, &s.1))
        .max()
        .expect("incorrect inputs");

    return (
        cmp::min(sensor_bounds.0, beacon_bounds.0) - largest_distance,
        cmp::min(sensor_bounds.1, beacon_bounds.1) - largest_distance,
        cmp::max(sensor_bounds.2, beacon_bounds.2) + largest_distance,
        cmp::max(sensor_bounds.3, beacon_bounds.3) + largest_distance,
    );
}

fn normalize(v: &mut Vector2D<i64>) {
    if v.x.abs() > 1 {
        v.x = v.x / v.x.abs();
    }

    if v.y.abs() > 1 {
        v.y = v.y / v.y.abs();
    }
}
