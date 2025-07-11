use chrono::{DateTime, Utc};
use colored::Colorize;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct ShouldIDeploy {
    timezone: String,
    date: DateTime<Utc>,
    shouldideploy: bool,
    message: String,
}

fn get_shouldideploytoday() -> Result<ShouldIDeploy, Box<dyn Error>> {
    let url = "https://shouldideploy.today/api";
    let now_utc = Utc::now();
    let formatted_date = now_utc.format("%Y-%m-%dT%H:%M:%S.000Z").to_string();

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let response = client
        .get(url)
        .query(&[("tz", "UTC"), ("date", &formatted_date)])
        .send()?
        .error_for_status()?;
    let parsed_response = response.json::<ShouldIDeploy>()?;

    Ok(parsed_response)
}

pub fn shouldideploytoday() {
    let shouldideploytoday = get_shouldideploytoday().unwrap();

    if shouldideploytoday.shouldideploy {
        println!("{}", &shouldideploytoday.message.bright_green());
    } else {
        println!("{}", &shouldideploytoday.message.bright_red());
    }
}
