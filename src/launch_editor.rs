use serde_derive::{Deserialize, Serialize};
use std::{fmt, fs};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct LaunchConfigurationCargoConfigFilter {
    name: String,
    kind: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct LaunchConfigurationCargoConfig {
    args: Vec<String>,
    filter: LaunchConfigurationCargoConfigFilter,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct LaunchConfiguration {
    r#type: String,
    request: String,
    name: String,
    cargo: LaunchConfigurationCargoConfig,
    args: Vec<String>,
    cwd: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LaunchConfigurations {
    configurations: Vec<LaunchConfiguration>,
}

#[derive(Debug, Clone)]
pub struct LaunchEditorError {
    e: String,
}

impl fmt::Display for LaunchEditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not edit Launch parameters: {}", self.e)
    }
}

const LAUNCH_JSON_PATH: &str = ".vscode/launch.json";

pub fn create_day_launch(day: i32) -> Result<bool, LaunchEditorError> {
    let launch_json_string =
        fs::read_to_string(LAUNCH_JSON_PATH).expect("could not open launch.json");

    let configurations_result = serde_json::from_str::<LaunchConfigurations>(&launch_json_string);
    let configuration_container = match configurations_result {
        Ok(configurations) => configurations,
        Err(error) => {
            return Err(LaunchEditorError {
                e: error.to_string(),
            })
        }
    };

    let day_name = format!("Debug Day {}", day);

    {
        let configuration_ref = &configuration_container.configurations;
        for configuration in configuration_ref {
            if configuration.name == day_name {
                println!("launch config for day {} already exists.", day);
                return Ok(false);
            }
        }
    }

    let new_configuration = LaunchConfiguration {
        r#type: "lldb".to_string(),
        request: "launch".to_string(),
        name: day_name,
        cargo: LaunchConfigurationCargoConfig {
            args: vec![
                "build".to_string(),
                "--bin=advent_of_code_2022".to_string(),
                "--package=advent_of_code_2022".to_string(),
            ],
            filter: LaunchConfigurationCargoConfigFilter {
                name: "advent_of_code_2022".to_string(),
                kind: "bin".to_string(),
            },
        },
        args: vec!["-day".to_string(), day.to_string()],
        cwd: "${workspaceFolder}".to_string(),
    };

    let mut new_configuration_container = LaunchConfigurations {
        configurations: configuration_container.configurations.to_vec(),
    };

    new_configuration_container
        .configurations
        .push(new_configuration);

    let new_launch_json_string_result = serde_json::to_string_pretty(&new_configuration_container);
    let new_launch_json_string = match new_launch_json_string_result {
        Ok(new_launch_json_string) => new_launch_json_string,
        Err(_) => {
            return Err(LaunchEditorError {
                e: String::from("Could not serialize new launch configurations."),
            })
        }
    };

    let write_result = fs::write(LAUNCH_JSON_PATH, new_launch_json_string);
    match write_result {
        Ok(_) => return Ok(true),
        Err(error) => {
            return Err(LaunchEditorError {
                e: error.to_string(),
            })
        }
    };
}
