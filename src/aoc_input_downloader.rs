use std::{fmt, fs};

#[derive(Debug, Clone)]
pub struct InputDownloadError {
    pub e: String,
}

impl fmt::Display for InputDownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not download input: {}", self.e)
    }
}

use reqwest::header::COOKIE;

const BASE_URL: &str = "https://adventofcode.com";
const YEAR: &str = "2022";
const COOKIE_PATH: &str = "./secrets/aoc_cookie.txt";

const INPUT_PATH: &str = "./src/inputs";

pub async fn download_input(day: i32) -> Result<(), InputDownloadError> {
    let cookie_value =
        fs::read_to_string(COOKIE_PATH).expect(format!("Could not open {}", COOKIE_PATH).as_str());

    let request_url = format!("{}/{}/day/{}/input", BASE_URL, YEAR, day.to_string());

    println!("requesting [{}]", request_url);

    let client = reqwest::Client::new();
    let response_result = client
        .get(request_url)
        .header(COOKIE, cookie_value)
        .send()
        .await;

    let response = match response_result {
        Ok(response) => response,
        Err(error) => {
            return Err(InputDownloadError {
                e: error.to_string(),
            })
        }
    };

    let input_result = response.text().await;
    let input = match input_result {
        Ok(input) => input,
        Err(error) => {
            return Err(InputDownloadError {
                e: error.to_string(),
            })
        }
    };

    let file_path = format!("{}/input_day_{}.txt", INPUT_PATH, day.to_string());
    if let Err(error) = fs::write(file_path, input) {
        return Err(InputDownloadError {
            e: error.to_string(),
        });
    }

    Ok(())
}
