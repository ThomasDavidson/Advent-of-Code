use reqwest::{Client, Response, StatusCode, Url};
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

const USER_AGENT: &str = "github.com/ThomasDavidson/Advent-of-Code by endershadow909@gmail.com";

fn generate_file_path(year: u16, day: u8) -> PathBuf {
    PathBuf::from(format!("./{year}/day_{day}/input.txt"))
}

fn generate_download_link(year: u16, day: u8) -> String {
    format!("https://adventofcode.com/{year}/day/{day}/input")
}

async fn request_input(session_cookie: &[u8], url: Url) -> Response {
    Client::new()
        .get(url)
        .header("Cookie", session_cookie)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .expect("Download failed, add session id to file if you haven't")
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let session_cookie = fs::read("session.txt")?;

    let years = 2023..=2024;
    let days = 1..=25;

    for year in years {
        for day in days.clone() {
            let path = generate_file_path(year, day);
            let download_link = generate_download_link(year, day);

            // remove file if it already exists
            if let Ok(exists) = fs::exists(&path) {
                if exists {
                    fs::remove_file(&path)?;
                }
            }

            // get file

            let url = Url::from_str(&download_link).unwrap();

            let response = request_input(&session_cookie, url).await;

            if response.status() == StatusCode::BAD_REQUEST {
                panic!("Bad Request: Check format of session file")
            }

            let text = response.text().await.unwrap();

            match fs::write(path, text) {
                Err(_) => {
                    println!("Can't write file check path");
                    break;
                }
                Ok(_) => println!("Success"),
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    Ok(())
}
