use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("this is ok for now");

    // part 1
    let lines = input.lines();
    let mut sums: Vec<i32> = vec![];
    let mut current_sum = 0;

    for line in lines {
        if line != "" {
            let calory = line.parse::<i32>().expect("this is ok for now");
            current_sum += calory;
        } else {
            sums.push(current_sum);
            current_sum = 0;
        }
    }
    sums.push(current_sum);

    let final_sums = sums.clone();

    let mut max_value = i32::MIN;

    for sum in final_sums.into_iter() {
        if sum > max_value {
            max_value = sum;
        }
    }

    println!("{}", max_value);

    // part 2
    let mut top_three = 0;
    sums.sort();
    for _ in 0..3 {
        top_three += sums.pop().expect("iz ok");
    }

    println!("{}", top_three);
}
