use std::{fs, io::Error};

const DAY_WILDCARD: &str = "_DAYNUMBER_";
const DAY_TEMPLATE_PATH: &str = "src/aoc_day/templates/day_template";
const NEW_DAY_PATH: &str = "src/aoc_day";
const DAY_MOD_PATH: &str = "src/aoc_day/mod.rs";

pub trait AOCDay {
    fn run_part_1(&self) -> &str;
    fn run_part_2(&self) -> &str;
}

pub fn create_aoc_day_source_file(day: i32) -> Result<(), Error> {
    // open template
    let day_template_original = fs::read_to_string(DAY_TEMPLATE_PATH)
        .expect(format!("Could not open {}", DAY_TEMPLATE_PATH).as_str());

    // write template in a new file.
    let new_day = day_template_original.replace(DAY_WILDCARD, day.to_string().as_str());
    let new_day_file_path = format!("{}/{}.rs", NEW_DAY_PATH, get_formatted_module_name(day));
    {
        let new_file_path_ref = &new_day_file_path;
        if let Err(e) = fs::write(new_file_path_ref, new_day) {
            return Err(e);
        }
    }

    // read aoc_day's mod.rs
    let mod_rs = match fs::read_to_string(DAY_MOD_PATH) {
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
        Some(_) => mod_rs,
    };

    // write the content to the file.
    if let Err(e) = fs::write(DAY_MOD_PATH, new_mod_rs) {
        delete_file_with_expect(&new_day_file_path);
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

fn delete_file_with_expect(path: &String) {
    fs::remove_file(path).expect(format!("Could not clean up file created at {}", path).as_str());
}
