use std::{
    fs,
    io::{self, Error},
};

use crate::config;

const DAY_WILDCARD: &str = "_DAYNUMBER_";
const DAY_FACTORY_MARKER: &str = "//_ADDADAY_";

pub trait AOCDayRunner {
    fn run_part_1(&self, input: &String, test_input: &String) -> String;
    fn run_part_2(&self, input: &String, test_input: &String) -> String;
}

pub fn create_aoc_day_source_file(day: i32) -> Result<(), Error> {
    println!("Duplicating day template");

    // open template
    let day_template_original = fs::read_to_string(config::DAY_TEMPLATE_PATH)
        .expect(format!("Could not open {}", config::DAY_TEMPLATE_PATH).as_str());

    // write template in a new file.
    let new_day = day_template_original.replace(DAY_WILDCARD, day.to_string().as_str());
    let new_day_file_path = format!(
        "{}/{}.rs",
        config::NEW_DAY_PATH,
        get_formatted_module_name(day)
    );
    {
        let new_file_path_ref = &new_day_file_path;
        if let Err(e) = fs::write(new_file_path_ref, new_day) {
            return Err(e);
        }
    }

    // read aoc_day's mod.rs
    println!("updating {}...", config::DAY_MOD_PATH);

    let mod_rs = match fs::read_to_string(config::DAY_MOD_PATH) {
        Ok(mod_rs) => mod_rs,
        Err(e) => {
            // clean up created file.
            delete_file_with_expect(&new_day_file_path);
            return Err(e);
        }
    };

    // edit the content if necessary.
    let new_mod_rs = match mod_rs
        .lines()
        .position(|line| line == get_formatted_module_include(day).as_str())
    {
        None => format!("{}\n{}", mod_rs, get_formatted_module_include(day)),
        Some(_) => mod_rs.clone(),
    };

    // write the content to the file.
    if let Err(e) = fs::write(config::DAY_MOD_PATH, new_mod_rs) {
        delete_file_with_expect(&new_day_file_path);
        return Err(e);
    };

    println!("{} updated", config::DAY_MOD_PATH);

    // adding day to the day factory
    println!("updating {}...", config::DAY_RUNNER_FACTORY_PATH);
    let aoc_day_factory_rs = match fs::read_to_string(config::DAY_RUNNER_FACTORY_PATH) {
        Ok(aoc_day_factory_rs) => aoc_day_factory_rs,
        Err(e) => {
            // clean up created file.
            delete_file_with_expect(&new_day_file_path);
            // revert edited files
            write_file_with_expect(config::DAY_MOD_PATH, &mod_rs);
            return Err(e);
        }
    };

    let mut aoc_day_factory_rs_lines: Vec<&str> = aoc_day_factory_rs.lines().collect();
    let mut target_line_index = usize::MAX;
    let day_factory_marker_str = DAY_FACTORY_MARKER.to_string();

    // Find the new factory line marker
    for i in 0..aoc_day_factory_rs_lines.len() {
        let original: String = aoc_day_factory_rs_lines[i]
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        if original == day_factory_marker_str {
            target_line_index = i;
        }
    }

    // Could not find it, clean up and return Err.
    if target_line_index == usize::MAX {
        // clean up created file.
        delete_file_with_expect(&new_day_file_path);
        // revert edited files
        write_file_with_expect(config::DAY_MOD_PATH, &mod_rs);

        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Could not find marker in {}",
                config::DAY_RUNNER_FACTORY_PATH
            ),
        ));
    }

    let day_str = day.to_string();
    let new_line = format!(
        "        {} => Some(Box::new(aoc_day::aoc_day_{}::DayRunner{} {{}})),",
        day_str, day_str, day_str
    );

    aoc_day_factory_rs_lines.insert(target_line_index + 1, new_line.as_str());

    let new_aoc_day_factory_rs = aoc_day_factory_rs_lines.join("\n");

    // write the content to the file.
    if let Err(e) = fs::write(config::DAY_RUNNER_FACTORY_PATH, new_aoc_day_factory_rs) {
        // clean up created file.
        delete_file_with_expect(&new_day_file_path);
        // revert edited files
        write_file_with_expect(config::DAY_MOD_PATH, &mod_rs);
        return Err(e);
    };

    Ok(())
}

fn get_formatted_module_include(day: i32) -> String {
    return format!("pub mod {};", get_formatted_module_name(day));
}

fn get_formatted_module_name(day: i32) -> String {
    return format!("aoc_day_{}", day);
}

// clean up helpers
fn delete_file_with_expect(path: &String) {
    fs::remove_file(path).expect(format!("Could not clean up file created at {}", path).as_str());
}

fn write_file_with_expect(path: &str, contents: &String) {
    fs::write(path, contents).expect(format!("Could not restore file {}", path).as_str())
}
