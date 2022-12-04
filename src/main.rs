mod aoc_args;
mod aoc_day;
mod aoc_input_downloader;
mod config;
mod launch_editor;

use aoc_args::{ArgumentOptions, Arguments};
use std::{env, fs};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match aoc_args::extract_args(args) {
        Ok(arguments) => process_arguments(arguments).await,
        Err(e) => eprint!("{}", e),
    }
}

async fn process_arguments(arguments: Arguments) {
    match arguments.option {
        ArgumentOptions::Day => run_day(arguments.values[0]),
        ArgumentOptions::Download => download_day(arguments.values[0]).await,
        ArgumentOptions::Create => create_day(arguments.values[0]).await,
    }
}

fn run_day(day: i32) {
    println!("Running day {}...", day);

    let input_path = config::get_formatted_input_file_path(day);
    let input = match fs::read_to_string(input_path) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Cannot run day {}, {}", day.to_string(), e);
            return;
        }
    };

    let test_input_path = config::get_formatted_test_input_file_path(day);
    let test_input = match fs::read_to_string(test_input_path) {
        Ok(test_input) => test_input,
        Err(e) => {
            eprintln!("Cannot runt day {}, {}", day.to_string(), e);
            return;
        }
    };

    match aoc_day::aoc_day_runner_factory::create_day_runner(day) {
        Some(day_runner) => {
            println!("------------------------------------------");
            println!("part 1: {}", day_runner.run_part_1(&input, &test_input));
            println!("------------------------------------------");
            println!("part 2: {}", day_runner.run_part_2(&input, &test_input));
            println!("------------------------------------------");
        }
        None => {
            eprintln!("Could not create day runner.");
            return;
        }
    };
}

async fn download_day(day: i32) {
    println!("downloading day {}...", day);
    match aoc_input_downloader::download_input(day).await {
        Ok(_) => println!("done!"),
        Err(e) => eprintln!("{}", e),
    };
}

async fn create_day(day: i32) {
    println!("creating day {}...", day);

    download_day(day).await;

    // create a launch configuration in launch.json
    match launch_editor::create_day_launch(day) {
        Ok(did_add_new_config) => {
            if did_add_new_config {
                println!("Added launch configuration.")
            }
        }
        Err(e) => {
            eprint!(
                "Could not create launch configuration for day {}, {}",
                day, e
            );
            return;
        }
    };

    // create a source file for that specific day
    match aoc_day::aoc_day::create_aoc_day_source_file(day) {
        Ok(_) => println!("Created day's source files"),
        Err(e) => {
            eprint!("Could not create source file for day {}, {}", day, e);
            return;
        }
    };
}
