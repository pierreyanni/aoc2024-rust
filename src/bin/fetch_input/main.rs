use std::fs::File;
use std::io::Write;
use reqwest;
use tokio;
use std::env;

async fn get_input(url: &str, session_cookie: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create a client
    let client = reqwest::Client::new();

    // Fetch the data asynchronously with authentication
    let response = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={}", session_cookie))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

async fn save_input(day: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    // Your session cookie
    let session_cookie = env::var("AOC_SESSION").expect("AOC_SESSION environment variable not set");

    // Fetch the input data
    let response = get_input(&url, &session_cookie).await?;

    // Save the data to a file
    let mut file = File::create(format!("input/day{:02}.txt",  day.parse::<u32>().unwrap()))?;
    file.write_all(response.as_bytes())?;

    println!("Data saved to input/day{:02}.txt", day.parse::<u32>().unwrap());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the day from command-line arguments
    let args: Vec<String> = env::args().collect();
    let day = if args.len() > 1 {
        &args[1]
    } else {
        "01" // Default to day 1 if no argument is provided
    };

    // Call save_input with the specified day
    save_input(day).await
}