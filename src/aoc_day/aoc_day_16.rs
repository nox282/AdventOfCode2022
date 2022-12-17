use std::collections::{HashMap, HashSet};

use crate::aoc_day;

pub struct DayRunner16 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner16 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let (flow_rates, graph) = parse_input(input);
        let distances = floyd_warshall(&graph);
        let positive_flow_valves: HashMap<&String, &i64> =
            flow_rates.iter().filter(|kvp| kvp.1 > &0).collect();
        return format!(
            "{}",
            dfs(
                &"AA".to_string(),
                0,
                &HashSet::new(),
                &positive_flow_valves,
                &distances,
                30
            )
        );
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let (flow_rates, graph) = parse_input(input);
        let distances = floyd_warshall(&graph);
        let positive_flow_valves: HashMap<&String, &i64> =
            flow_rates.iter().filter(|kvp| kvp.1 > &0).collect();

        return format!(
            "{}",
            dfs_with_2_agents(
                &"AA".to_string(),
                0,
                &"AA".to_string(),
                0,
                &HashSet::new(),
                &positive_flow_valves,
                &distances,
                26,
                &mut HashMap::new(),
            )
        );
    }
}

fn parse_input(input: &str) -> (HashMap<String, i64>, HashMap<String, Vec<String>>) {
    let mut flow_rates: HashMap<String, i64> = HashMap::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    input.lines().for_each(|line| {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let cleaned_up_line = line
            .replace("Valve ", "")
            .replace(" has flow rate=", ",")
            .replace(" tunnels lead to valves ", "")
            .replace(" tunnel leads to valve ", "");

        let splits: Vec<&str> = cleaned_up_line.split(";").collect();
        assert_eq!(splits.len(), 2);

        let flow_rate_raw: Vec<&str> = splits[0].split(",").collect();
        assert_eq!(flow_rate_raw.len(), 2);
        flow_rates.insert(
            flow_rate_raw[0].to_string(),
            flow_rate_raw[1].parse::<i64>().expect("incorrect inputs."),
        );

        let cleaned_up_split_1 = splits[1].replace(" ", "");
        let tunnels: Vec<String> = cleaned_up_split_1
            .split(",")
            .map(|s| s.to_string())
            .collect();
        graph.insert(flow_rate_raw[0].to_string(), tunnels);
    });

    return (flow_rates, graph);
}

// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
fn floyd_warshall(graph: &HashMap<String, Vec<String>>) -> HashMap<(&String, &String), i64> {
    let mut distances: HashMap<(&String, &String), i64> = HashMap::new();
    for (valve, neighbors) in graph.iter() {
        for neighbor in neighbors.iter() {
            distances.insert((valve, neighbor), 1);
        }
        distances.insert((valve, valve), 0);
    }

    for (k, _) in graph.iter() {
        for (i, _) in graph.iter() {
            if !distances.contains_key(&(i, k)) {
                continue;
            }

            for (j, _) in graph.iter() {
                if !distances.contains_key(&(k, j)) {
                    continue;
                }

                if distances.contains_key(&(i, j)) {
                    distances.insert(
                        (i, j),
                        std::cmp::min(distances[&(i, j)], distances[&(i, k)] + distances[&(k, j)]),
                    );
                } else {
                    distances.insert((i, j), distances[&(i, k)] + distances[&(k, j)]);
                }
            }
        }
    }

    return distances;
}

fn dfs(
    current_valve: &String,
    depth: i64,
    opened_valves: &HashSet<&String>,
    positive_flow_valves: &HashMap<&String, &i64>,
    distances: &HashMap<(&String, &String), i64>,
    max_depth: i64,
) -> i64 {
    if opened_valves.len() == positive_flow_valves.len() {
        return 0;
    }

    let mut pressure_released: i64 = 0;
    for (valve, flow_rate) in positive_flow_valves {
        if opened_valves.contains(valve) {
            continue;
        }

        let cost = distances[&(current_valve, *valve)] + 1;
        if depth + cost <= max_depth {
            let released_pressure = *flow_rate * (max_depth - (depth + cost));

            let mut new_opened_valves = opened_valves.clone();
            new_opened_valves.insert(valve);

            let sub_pressure_released = dfs(
                valve,
                depth + cost,
                &new_opened_valves,
                positive_flow_valves,
                distances,
                max_depth,
            );

            pressure_released =
                std::cmp::max(sub_pressure_released + released_pressure, pressure_released);
        }
    }

    return pressure_released;
}

fn dfs_with_2_agents(
    first_current_valve: &String,
    first_depth: i64,
    second_current_valve: &String,
    second_depth: i64,
    opened_valves: &HashSet<&String>,
    positive_flow_valves: &HashMap<&String, &i64>,
    distances: &HashMap<(&String, &String), i64>,
    max_depth: i64,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if opened_valves.len() == positive_flow_valves.len() {
        return 0;
    }

    let first_memo_key = format!(
        "{:?}{}{}{}{}",
        opened_valves, first_current_valve, first_depth, second_current_valve, second_depth
    );
    let second_memo_key = format!(
        "{:?}{}{}{}{}",
        opened_valves, second_current_valve, second_depth, first_current_valve, first_depth
    );

    if memo.contains_key(&first_memo_key) {
        return memo[&first_memo_key];
    }

    if memo.contains_key(&second_memo_key) {
        return memo[&second_memo_key];
    }

    let mut pressure_released: i64 = 0;
    for (valve, flow_rate) in positive_flow_valves {
        if opened_valves.contains(valve) {
            continue;
        }

        let first_cost = first_depth + distances[&(first_current_valve, *valve)] + 1;
        let second_cost = second_depth + distances[&(second_current_valve, *valve)] + 1;

        if first_cost <= second_cost {
            if first_cost <= max_depth {
                let released_pressure = *flow_rate * (max_depth - first_cost);

                let mut new_opened_valves = opened_valves.clone();
                new_opened_valves.insert(valve);

                let sub_pressure_released = dfs_with_2_agents(
                    valve,
                    first_cost,
                    second_current_valve,
                    second_depth,
                    &new_opened_valves,
                    positive_flow_valves,
                    distances,
                    max_depth,
                    memo,
                );

                pressure_released =
                    std::cmp::max(sub_pressure_released + released_pressure, pressure_released);
            }
        }

        if second_cost <= first_cost {
            if second_cost <= max_depth {
                let released_pressure = *flow_rate * (max_depth - second_cost);

                let mut new_opened_valves = opened_valves.clone();
                new_opened_valves.insert(valve);

                let sub_pressure_released = dfs_with_2_agents(
                    first_current_valve,
                    first_depth,
                    valve,
                    second_cost,
                    &new_opened_valves,
                    positive_flow_valves,
                    distances,
                    max_depth,
                    memo,
                );

                pressure_released =
                    std::cmp::max(sub_pressure_released + released_pressure, pressure_released);
            }
        }
    }

    memo.insert(first_memo_key, pressure_released);
    memo.insert(second_memo_key, pressure_released);

    return pressure_released;
}
