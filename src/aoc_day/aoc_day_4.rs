use crate::aoc_day;

pub struct DayRunner4 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner4 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let contained_assignements: Vec<_> = input
            .lines()
            .filter(|line| {
                let pairs = parse_line(line);

                return does_pair1_contain_pair2(&pairs.0, &pairs.1)
                    || does_pair1_contain_pair2(&pairs.1, &pairs.0);
            })
            .collect();
        return format!(
            "contained assignement count: {}",
            contained_assignements.len()
        );
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let overlapping_assignements: Vec<_> = input
            .lines()
            .filter(|line| {
                let pairs = parse_line(line);
                return do_pairs_overlap(&pairs.0, &pairs.1);
            })
            .collect();
        return format!(
            "overlapping assignements count: {}",
            overlapping_assignements.len()
        );
    }
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let pairs: Vec<&str> = line.split(",").collect();
    assert_eq!(pairs.len(), 2);

    let pair1: Vec<i32> = pairs[0]
        .split("-")
        .map(|e| e.parse::<i32>().expect("Inconrrect inputs"))
        .collect();
    assert_eq!(pair1.len(), 2);

    let pair2: Vec<i32> = pairs[1]
        .split("-")
        .map(|e| e.parse::<i32>().expect("Inconrrect inputs"))
        .collect();
    assert_eq!(pair2.len(), 2);

    return ((pair1[0], pair1[1]), (pair2[0], pair2[1]));
}

fn do_pairs_overlap(pair1: &(i32, i32), pair2: &(i32, i32)) -> bool {
    // pair1 ....[...]....
    // pair2 ......[].....

    // pair1 ....[].......
    // pair2 ...[...].....

    // pair1 ....[...]....
    // pair2 ......[...]..

    // pair1 ....[....]...
    // pair2 ..[...]......

    return does_pair1_contain_pair2(&pair1, &pair2)
        || does_pair1_contain_pair2(&pair2, &pair1)
        || pair1.0 <= pair2.1 && pair1.1 >= pair2.0;
}

fn does_pair1_contain_pair2(pair1: &(i32, i32), pair2: &(i32, i32)) -> bool {
    // pair1 ....[...]....
    // pair2 ......[].....
    return pair1.0 <= pair2.0 && pair1.1 >= pair2.1;
}
