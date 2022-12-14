use vector2d::Vector2D;

use crate::aoc_day;

use std::{cmp, collections::HashMap};

type Point = (i32, i32);
type Rect = (i32, i32, i32, i32);

const SOLID: char = '#';
const SAND: char = 'o';
const FALLING_SAND: char = '*';

const GRAVITY: Vector2D<i32> = Vector2D::<i32> { x: 0, y: -1 };
const LEFT: Vector2D<i32> = Vector2D::<i32> { x: -1, y: 0 };
const RIGHT: Vector2D<i32> = Vector2D::<i32> { x: 1, y: 0 };

struct World {
    entities: HashMap<Point, char>,
    bounds: Rect,
    has_floor: bool,
}

pub struct DayRunner14 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner14 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let mut world = parse_input(input, false);

        let sand_source = (500, 0);
        let mut sand_point = sand_source;
        loop {
            sand_point = tick(&mut world, &sand_point);

            if world.entities[&sand_point] == SAND {
                sand_point = sand_source;
            }

            if sand_point.1 >= world.bounds.3 {
                break;
            }

            // // uncomment for cool visualization
            // {
            //     use core::time;
            //     use std::thread;

            //     display(&world, (475, 0), (525, 75), 0);
            //     let sixteen_millis = time::Duration::from_millis(16);
            //     thread::sleep(sixteen_millis);
            // }
        }

        let ans = world.entities.iter().filter(|kvp| kvp.1 == &SAND).count();
        return format!("{}", ans);
    }

    fn run_part_2(&self, input: &String, test_input: &String) -> String {
        let mut world = parse_input(input, true);

        world.bounds.3 += 2;

        let sand_source = (500, 0);
        let mut sand_point = sand_source;

        loop {
            match world.entities.get(&sand_source) {
                Some(c) => {
                    if c == &SAND {
                        break;
                    }
                }
                None => {}
            }

            sand_point = tick(&mut world, &sand_point);
            if world.entities[&sand_point] == SAND {
                sand_point = sand_source;
            }

            // // uncomment for cool visualization
            // {
            //     use core::time;
            //     use std::thread;

            //     display(&world, (475, 0), (525, 75), 0);
            //     let sixteen_millis = time::Duration::from_millis(16);
            //     thread::sleep(sixteen_millis);
            // }
        }

        let ans = world.entities.iter().filter(|kvp| kvp.1 == &SAND).count();
        return format!("{}", ans);
    }
}

fn parse_input(input: &str, has_floor: bool) -> World {
    let mut world = World {
        entities: HashMap::new(),
        bounds: (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        has_floor,
    };

    input.lines().for_each(|line| {
        let vectors: Vec<Vector2D<i32>> = line
            .split(" -> ")
            .map(|raw_points| {
                let components: Vec<&str> = raw_points.split(",").collect();
                assert_eq!(components.len(), 2);
                let x = components[0].parse::<i32>().expect("incorrect inputs.");
                let y = components[1].parse::<i32>().expect("incorrect inputs.");

                world.bounds.0 = cmp::min(world.bounds.0, x);
                world.bounds.1 = cmp::min(world.bounds.1, y);
                world.bounds.2 = cmp::max(world.bounds.2, x);
                world.bounds.3 = cmp::max(world.bounds.3, y);

                return Vector2D::<i32> { x, y };
            })
            .collect();

        for i in 0..(vectors.len() - 1) {
            let v1 = vectors[i];
            let v2 = vectors[i + 1];

            let mut delta = v2 - v1;
            normalize(&mut delta);

            let mut current = v1;
            while current != v2 {
                world.entities.insert(vector2d_to_point(&current), SOLID);
                current += delta;
            }

            world.entities.insert(vector2d_to_point(&v2), SOLID);
        }
    });

    return world;
}

#[allow(dead_code)]
fn display(world: &World, from: Point, to: Point, padding: i32) {
    assert!(from.0 <= to.0);
    assert!(from.1 <= to.1);

    const AIR: char = '.';

    // clear std out
    clearscreen::clear().expect("failed to clear screen");

    for y in from.1 - padding..to.1 + padding {
        for x in from.0 - padding..to.0 + padding {
            if world.has_floor && y >= world.bounds.3 {
                print!("{}", SOLID);
            } else {
                let c = (x, y);
                match world.entities.get(&c) {
                    Some(entity) => print!("{}", entity),
                    None => print!("{}", AIR),
                }
            }
        }
        print!("\n");
    }
}

fn normalize(v: &mut Vector2D<i32>) {
    if v.x.abs() > 1 {
        v.x = v.x / v.x.abs();
    }

    if v.y.abs() > 1 {
        v.y = v.y / v.y.abs();
    }
}

fn tick(world: &mut World, sand_point: &Point) -> Point {
    // update falling sand
    let old_sand_point = *sand_point;
    let new_sand_point = do_falling_sand_tick(world, &old_sand_point);

    if world.entities.contains_key(&old_sand_point) {
        move_entity(&mut world.entities, &old_sand_point, &new_sand_point);
    } else {
        world.entities.insert(new_sand_point, FALLING_SAND);
    }

    if new_sand_point == old_sand_point {
        *world.entities.entry(new_sand_point).or_insert(SAND) = SAND;
    }

    return new_sand_point;
}

fn point_to_vector2d(p: &Point) -> Vector2D<i32> {
    return Vector2D { x: p.0, y: p.1 };
}

fn vector2d_to_point(p: &Vector2D<i32>) -> Point {
    return (p.x, p.y);
}

fn do_falling_sand_tick(world: &World, sand_point: &Point) -> Point {
    let falling_sand_vec = point_to_vector2d(&sand_point);
    match get_entity(&world, &(falling_sand_vec - GRAVITY)) {
        None => {
            return vector2d_to_point(&(falling_sand_vec - GRAVITY));
        }
        Some(_) => {}
    }

    match get_entity(&world, &(falling_sand_vec - GRAVITY + LEFT)) {
        None => {
            return vector2d_to_point(&(falling_sand_vec - GRAVITY + LEFT));
        }
        Some(_) => {}
    }

    match get_entity(&world, &(falling_sand_vec - GRAVITY + RIGHT)) {
        None => {
            return vector2d_to_point(&(falling_sand_vec - GRAVITY + RIGHT));
        }
        Some(_) => {}
    }

    return *sand_point;
}

fn get_entity(world: &World, v: &Vector2D<i32>) -> Option<char> {
    let point = &vector2d_to_point(&v);

    if world.has_floor && point.1 >= world.bounds.3 {
        return Some('#');
    }

    return match world.entities.get(point) {
        Some(c) => Some(*c),
        None => None,
    };
}

fn move_entity(entities: &mut HashMap<Point, char>, old: &Point, new: &Point) {
    if entities.contains_key(old) {
        let e = entities[old];
        entities.remove(old);
        entities.insert(*new, e);
    }
}
