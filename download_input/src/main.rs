use reqwest::header::USER_AGENT;
use reqwest::{Client, Url};
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fs, io, thread};

fn generate_file_path(year: u16, day: u8) -> PathBuf {
    PathBuf::from(format!("./{year}/day_{day}/input.txt"))
}

fn generate_download_link(year: u16, day: u8) -> String {
    format!("https://adventofcode.com/{year}/day/{day}/input")
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

            let response = Client::new()
                .get(url)
                .header("Cookie", session_cookie.clone())
                .send()
                .await
                .expect("Download failed, add session id to file if you havn't");

            let text = response.text().await.unwrap();

            match fs::write(path, text) {
                Err(_) => {
                    println!("Can't write file check path");
                    break;
                }
                Ok(_) => (),
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    Ok(())
}
