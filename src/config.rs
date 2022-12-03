// inputs
pub const INPUTS_PATH: &str = "src/inputs/day_";
pub const INPUT_FILE_PREFIX: &str = "input_day_";
pub const TEST_INPUT_FILE_PREFIX: &str = "test_input_day_";

// secrets
pub const COOKIE_PATH: &str = "secrets/aoc_cookie.txt";

// env
pub const LAUNCH_JSON_PATH: &str = ".vscode/launch.json";

// day
pub const DAY_TEMPLATE_PATH: &str = "src/aoc_day/templates/day_template";
pub const NEW_DAY_PATH: &str = "src/aoc_day";
pub const DAY_MOD_PATH: &str = "src/aoc_day/mod.rs";
pub const DAY_RUNNER_FACTORY_PATH: &str = "src/aoc_day/aoc_day_runner_factory.rs";

pub fn get_formatted_input_directory_path(day: i32) -> String {
    let day_string = day.to_string();
    return format!("{}{}", INPUTS_PATH, day_string);
}

pub fn get_formatted_input_file_path(day: i32) -> String {
    let day_string = day.to_string();
    return format!(
        "{}/{}{}",
        get_formatted_input_directory_path(day),
        INPUT_FILE_PREFIX,
        day_string
    );
}

pub fn get_formatted_test_input_file_path(day: i32) -> String {
    let day_string = day.to_string();
    return format!(
        "{}/{}{}",
        get_formatted_input_directory_path(day),
        TEST_INPUT_FILE_PREFIX,
        day_string
    );
}
