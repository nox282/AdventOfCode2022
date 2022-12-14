use std::cmp::Ordering;

use crate::aoc_day;

pub struct DayRunner13 {}

impl aoc_day::aoc_day::AOCDayRunner for DayRunner13 {
    fn run_part_1(&self, input: &String, _: &String) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let res = lines.chunks(3).enumerate().fold(0, |acc, e| {
            let index = e.0 + 1;
            let chunk = e.1;

            match compare(&chunk[0], &chunk[1]) {
                Ordering::Less => {
                    print!("{},", index);
                    return acc + index;
                }
                _ => {}
            }

            return acc;
        });
        return format!("{}", res);
    }

    fn run_part_2(&self, input: &String, _: &String) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let mut packets: Vec<&str> = lines
            .chunks(3)
            .map(|chunk| vec![chunk[0], chunk[1]])
            .flatten()
            .collect();

        packets.push("[[2]]");
        packets.push("[[6]]");
        packets.sort_by(compare);

        let del_1 = match packets.iter().position(|p| p == &"[[2]]") {
            Some(index) => index,
            None => panic!("oh shit!"),
        } + 1;
        let del_2 = match packets.iter().position(|p| p == &"[[6]]") {
            Some(index) => index,
            None => panic!("oh shit!"),
        } + 1;

        return format!("{}", del_1 * del_2);
    }
}

fn compare<'r, 's>(l: &'r &str, r: &'s &str) -> Ordering {
    let mut left = l.to_string();
    let mut right = r.to_string();

    let mut l_i = 0;
    let mut r_i = 0;

    let max_size = std::cmp::max(left.len(), right.len());
    while l_i < max_size && r_i < max_size {
        let l_c = left.chars().nth(l_i);
        let r_c = right.chars().nth(r_i);

        match (l_c, r_c) {
            (Some('['), Some('[')) => {
                let l_closing_bracket_index = match find_closing_bracket_index(&left, l_i) {
                    Some(index) => index,
                    None => panic!("incorrect inputs"),
                };

                let r_closing_bracket_index = match find_closing_bracket_index(&right, r_i) {
                    Some(index) => index,
                    None => panic!("incorrect inputs"),
                };

                let sub_left = match l_closing_bracket_index - l_i <= 1 {
                    true => "",
                    false => &left[(l_i + 1)..(l_closing_bracket_index)],
                };

                let sub_right = match r_closing_bracket_index - r_i <= 1 {
                    true => "",
                    false => &right[(r_i + 1)..(r_closing_bracket_index)],
                };

                l_i = l_closing_bracket_index;
                r_i = r_closing_bracket_index;

                match compare(&sub_left, &sub_right) {
                    Ordering::Equal => {}
                    any_res => return any_res,
                }
            }

            (Some(']'), Some(']')) | (Some(','), Some(',')) => {
                l_i += 1;
                r_i += 1;
            }

            (Some(_), Some('[')) => {
                left = convert_section_to_list(&left, l_i);
            }

            (Some('['), Some(_)) => {
                right = convert_section_to_list(&right, r_i);
            }

            (None, Some(_)) => {
                return Ordering::Less;
            }

            (Some(_), None) => {
                return Ordering::Greater;
            }

            (Some(_), Some(_)) => {
                let (l_value, l_len) = get_value_at(&left, l_i);
                let (r_value, r_len) = get_value_at(&right, r_i);

                if l_value != r_value {
                    if l_value < r_value {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                }

                l_i += l_len;
                r_i += r_len;
            }

            (None, None) => {}
        };
    }

    return Ordering::Equal;
}

fn find_closing_bracket_index(s: &str, begin: usize) -> Option<usize> {
    let mut depth = 0;
    for i in begin..s.len() {
        let c = s.chars().nth(i);
        match c {
            Some('[') => depth += 1,
            Some(']') => depth -= 1,
            _ => {}
        }

        if depth == 0 {
            return Some(i);
        }
    }

    return None;
}

fn convert_section_to_list(s: &str, begin: usize) -> String {
    if s.len() == 0 {
        panic!("incorrect inputs");
    }

    let from = get_value_at(s, begin).0.to_string();
    let splits = s.split_at(begin);
    return format!(
        "{}{}",
        splits.0,
        splits.1.replacen(&from, &format!("[{}]", from), 1)
    );
}

fn get_value_at(s: &str, begin: usize) -> (i32, usize) {
    assert!(begin < s.len());

    if s.len() <= 1 || begin == s.len() - 1 {
        match s.chars().nth(begin).unwrap().to_string().parse::<i32>() {
            Ok(value) => return (value, 1),
            Err(_) => panic!("incorrect inputs"),
        };
    } else {
        let sub_s = &s[begin..s.len()];
        let end_of_value_index = match sub_s.find(",") {
            Some(index) => index,
            None => match sub_s.find("]") {
                Some(index) => index,
                None => sub_s.len(),
            },
        } + begin;

        match s[begin..end_of_value_index].to_string().parse::<i32>() {
            Ok(value) => (value, end_of_value_index - begin),
            Err(_) => panic!("incorrect inputs"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closing_bracket_index() {
        let p1 = "[]";
        assert_eq!(find_closing_bracket_index(p1, 0), Some(1));

        let p2 = "[[[[]]]]";
        assert_eq!(find_closing_bracket_index(p2, 2), Some(5));

        let p3 = "[][[[][]][][]]";
        assert_eq!(find_closing_bracket_index(p3, 2), Some(p3.len() - 1));
    }

    #[test]
    fn test_find_closing_bracket_index_when_none() {
        let p1 = "[[]";
        assert_eq!(find_closing_bracket_index(p1, 0), None);
    }

    #[test]
    fn test_convert_section_to_list() {
        let p1 = "1,2,3,4";

        assert_eq!(convert_section_to_list(p1, 0), "[1],2,3,4");
        assert_eq!(convert_section_to_list(p1, 2), "1,[2],3,4");
        assert_eq!(convert_section_to_list(p1, 4), "1,2,[3],4");
        assert_eq!(convert_section_to_list(p1, 6), "1,2,3,[4]");

        let p2 = "[[1,[2, 3]]]";
        assert_eq!(convert_section_to_list(p2, 5), "[[1,[[2], 3]]]");
    }
}
