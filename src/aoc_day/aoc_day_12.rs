use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Point = (usize, usize);

use crate::aoc_day;

pub struct DayRunner12 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner12 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let (grid, input_start, input_end) = parse_input(input);

        let mut came_from: HashMap<Point, Point> = HashMap::new();
        let start = bfs(
            &input_start,
            &grid,
            &mut came_from,
            |_, p| p.0 == input_end.0 && p.1 == input_end.1,
            &get_neighbors,
        );

        let ans = get_path_length(&start, &came_from) - 1;

        return format!("{}", ans);
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let (grid, _, input_end) = parse_input(input);

        let mut came_from: HashMap<Point, Point> = HashMap::new();
        let start = bfs(
            &input_end,
            &grid,
            &mut came_from,
            |grid, p| grid[p.0][p.1] == 'a',
            &get_down_neighbors,
        );

        let ans = get_path_length(&start, &came_from) - 1;

        return format!("{}", ans);
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Point, Point) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);

    input.lines().enumerate().for_each(|line| {
        let mut parsed_line: Vec<char> = vec![];
        line.1.chars().enumerate().for_each(|c| {
            let mut char = c.1;
            if char == 'S' {
                start = (line.0, c.0);
                char = 'a';
            }

            if char == 'E' {
                end = (line.0, c.0);
                char = 'z';
            }

            parsed_line.push(char);
        });

        if parsed_line.len() > 0 {
            grid.push(parsed_line);
        }
    });

    return (grid, start, end);
}

fn get_cardinal_neighbors(grid: &Vec<Vec<char>>, p: &Point) -> Vec<Point> {
    assert!(grid.len() > 0);
    let height = grid.len();
    let width = grid[0].len();

    let mut neighbors = vec![];

    if p.0 > 0 {
        neighbors.push((p.0 - 1, p.1));
    }

    if p.0 < height - 1 {
        neighbors.push((p.0 + 1, p.1));
    }

    if p.1 > 0 {
        neighbors.push((p.0, p.1 - 1));
    }

    if p.1 < width - 1 {
        neighbors.push((p.0, p.1 + 1));
    }

    return neighbors;
}

fn get_neighbors(grid: &Vec<Vec<char>>, p: &Point) -> Vec<Point> {
    let mut neighbors = get_cardinal_neighbors(grid, p);
    for i in (0..neighbors.len()).rev() {
        let c_value = grid[p.0][p.1] as i32;
        let n_value = grid[neighbors[i].0][neighbors[i].1] as i32;

        if n_value - c_value > 1 {
            neighbors.remove(i);
        }
    }

    return neighbors;
}

fn get_down_neighbors(grid: &Vec<Vec<char>>, p: &Point) -> Vec<Point> {
    let mut neighbors = get_cardinal_neighbors(grid, p);
    for i in (0..neighbors.len()).rev() {
        let c_value = grid[p.0][p.1] as i32;
        let n_value = grid[neighbors[i].0][neighbors[i].1] as i32;

        if c_value - n_value > 1 {
            neighbors.remove(i);
        }
    }

    return neighbors;
}

fn get_path_length(start: &Point, came_from: &HashMap<Point, Point>) -> usize {
    let mut current = Some(start);
    let mut n = 0;
    while current != None {
        current = came_from.get(current.unwrap());
        n += 1;
    }

    return n;
}

fn bfs(
    start: &Point,
    grid: &Vec<Vec<char>>,
    came_from: &mut HashMap<Point, Point>,
    has_reached_goal: impl Fn(&Vec<Vec<char>>, &Point) -> bool,
    get_neighbours: impl Fn(&Vec<Vec<char>>, &Point) -> Vec<Point>,
) -> Point {
    let mut distances: HashMap<Point, u32> = HashMap::new();
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            distances.insert((x, y), u32::MAX);
        }
    }

    let mut visited: HashSet<Point> = HashSet::new();
    let mut open_set = BinaryHeap::new();

    open_set.push((Reverse(0), *start));
    distances.insert(*start, 0);
    visited.insert(*start);

    while !open_set.is_empty() {
        let current = match open_set.pop() {
            Some(n) => n.1,
            None => continue,
        };

        if has_reached_goal(&grid, &current) {
            return current;
        }

        let neighbors = get_neighbours(&grid, &current);
        for neighbor in neighbors.into_iter() {
            if visited.contains(&neighbor) {
                continue;
            }

            visited.insert(neighbor);
            came_from.insert(neighbor, current);

            let neighbor_distance = distances[&current] + 1;
            distances.insert(neighbor, neighbor_distance);
            open_set.push((Reverse(neighbor_distance), neighbor));
        }
    }

    return (usize::MAX, usize::MAX);
}
