use std::fmt;

#[derive(Debug, Clone)]
pub struct ExtractArgumentError {
    pub e: String,
}

impl fmt::Display for ExtractArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Arguments: {}", self.e)
    }
}

#[derive(Debug, PartialEq)]
pub enum ArgumentOptions {
    Day,
    Download,
    Create,
}

#[derive(Debug)]
pub struct Arguments {
    pub option: ArgumentOptions,
    pub values: Vec<i32>,
}

pub fn extract_args(args: Vec<String>) -> Result<Arguments, ExtractArgumentError> {
    if args.len() < 1 {
        return Err(ExtractArgumentError {
            e: String::from("No arguments provided"),
        });
    }

    // find the first option index
    let option_index = find_index_of_next_option(&args);
    if option_index == usize::MAX {
        return Err(ExtractArgumentError {
            e: String::from("No option provided."),
        });
    }

    // split the array and get rid of everything before the option
    let options = args.split_at(option_index).1;
    if options.len() == 0 {
        return Err(ExtractArgumentError {
            e: String::from("Could not find an option in the provided arguments"),
        });
    }

    let option = &options[0];

    let mut arguments = Arguments {
        option: ArgumentOptions::Day,
        values: vec![],
    };

    // validate that the options matches our valid options
    match option.as_str() {
        "-day" => arguments.option = ArgumentOptions::Day,
        "-dl" => arguments.option = ArgumentOptions::Download,
        "-create" => arguments.option = ArgumentOptions::Create,
        _ => {
            return Err(ExtractArgumentError {
                e: String::from("No valid option provided."),
            })
        }
    }

    // validate the option's parameters
    match arguments.option {
        ArgumentOptions::Day | ArgumentOptions::Download | ArgumentOptions::Create => {
            if options.len() != 2 {
                return Err(ExtractArgumentError {
                    e: String::from("Missing day parameter."),
                });
            }

            match options[1].parse::<i32>() {
                Ok(day) => arguments.values.push(day),
                Err(_) => {
                    return Err(ExtractArgumentError {
                        e: String::from("Invalid day parameter."),
                    })
                }
            }
        }
    }

    Ok(arguments)
}

fn find_index_of_next_option(vec: &Vec<String>) -> usize {
    return vec
        .iter()
        .position(|arg| !arg.is_empty() && arg.chars().nth(0) == Some('-'))
        .unwrap_or(usize::MAX);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_args_expected() {
        for day in 1..25 {
            let args: Vec<String> = vec![String::from("-day"), day.to_string()];

            match extract_args(args) {
                Ok(arguments) => {
                    assert_eq!(arguments.option, ArgumentOptions::Day);
                    assert_eq!(arguments.values, vec![day]);
                }
                Err(_) => assert!(false),
            }
        }

        for day in 1..25 {
            let args: Vec<String> = vec![String::from("-dl"), day.to_string()];
            match extract_args(args) {
                Ok(arguments) => {
                    assert_eq!(arguments.option, ArgumentOptions::Download);
                    assert_eq!(arguments.values, vec![day]);
                }
                Err(_) => assert!(false),
            }
        }

        for day in 1..25 {
            let args: Vec<String> = vec![String::from("-create"), day.to_string()];
            match extract_args(args) {
                Ok(arguments) => {
                    assert_eq!(arguments.option, ArgumentOptions::Create);
                    assert_eq!(arguments.values, vec![day]);
                }
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn test_extract_args_wrong_amount_of_args_1_arg() {
        let args: Vec<String> = vec![String::from("1")];
        extract_args(args).expect_err("Test Failed");
    }

    #[test]
    fn test_extract_args_wrong_amount_of_args_2_args() {
        let args: Vec<String> = vec![String::from("1"), String::from("2")];
        extract_args(args).expect_err("Test Failed");
    }

    #[test]
    fn test_extract_args_wrong_first_option() {
        let args: Vec<String> = vec![
            String::from("random_shit"),
            String::from("1"),
            String::from("2"),
        ];

        extract_args(args).expect_err("Test Failed");
    }
}
