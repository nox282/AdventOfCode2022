use crate::aoc_day;

pub struct DayRunner2 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner2 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let total_points: i32 = input
            .lines()
            .map(|line| {
                let moves: Vec<&str> = line.split(" ").collect();
                assert_eq!(moves.len(), 2);
                let theirs = match match_move_to_rps(moves[0]) {
                    Some(rps) => rps,
                    None => panic!("incorrect input"),
                };

                let ours = match match_move_to_rps(moves[1]) {
                    Some(rps) => rps,
                    None => panic!("incorrect input"),
                };

                return get_match_point_value(&theirs, &ours) + get_rps_point_value(&ours);
            })
            .sum();
        return format!("total points {}", total_points);
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let total_points: i32 = input
            .lines()
            .map(|line| {
                let moves: Vec<&str> = line.split(" ").collect();
                assert_eq!(moves.len(), 2);
                let theirs = match match_move_to_rps(moves[0]) {
                    Some(rps) => rps,
                    None => panic!("incorrect input"),
                };

                let match_result = moves[1];
                let ours = match get_move_from_match_result(&theirs, match_result) {
                    Some(ours) => ours,
                    None => panic!("incorrect input"),
                };

                return get_match_point_value(&theirs, &ours) + get_rps_point_value(&ours);
            })
            .sum();
        return format!("total points {}", total_points);
    }
}

#[derive(Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn get_match_point_value(theirs: &RPS, ours: &RPS) -> i32 {
    return match (theirs, ours) {
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Paper, RPS::Rock) => 0,

        (RPS::Scissors, RPS::Rock) => 6,
        (RPS::Rock, RPS::Scissors) => 0,

        (RPS::Paper, RPS::Scissors) => 6,
        (RPS::Scissors, RPS::Paper) => 0,

        _ => 3,
    };
}

fn get_rps_point_value(v: &RPS) -> i32 {
    return match v {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
}

fn match_move_to_rps(m: &str) -> Option<RPS> {
    return match m {
        "A" => Some(RPS::Rock),
        "B" => Some(RPS::Paper),
        "C" => Some(RPS::Scissors),
        _ => match m {
            "X" => Some(RPS::Rock),
            "Y" => Some(RPS::Paper),
            "Z" => Some(RPS::Scissors),
            _ => None,
        },
    };
}

fn get_winning_move(theirs: &RPS) -> RPS {
    return match theirs {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissors,
        RPS::Scissors => RPS::Rock,
    };
}

fn get_losing_move(theirs: &RPS) -> RPS {
    return match theirs {
        RPS::Rock => RPS::Scissors,
        RPS::Paper => RPS::Rock,
        RPS::Scissors => RPS::Paper,
    };
}

fn get_draw_move(theirs: &RPS) -> RPS {
    return theirs.clone();
}

fn get_move_from_match_result(theirs: &RPS, match_result: &str) -> Option<RPS> {
    return match match_result {
        "X" => Some(get_losing_move(theirs)),
        "Y" => Some(get_draw_move(theirs)),
        "Z" => Some(get_winning_move(theirs)),
        _ => None,
    };
}
