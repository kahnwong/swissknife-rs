use colored::Colorize;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;
use std::io;
use std::net::UdpSocket;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct PublicIpResponse {
    ip: String,
}

#[derive(Deserialize, Debug)]
struct IpLocation {
    // ip: String,
    country: String,
    #[serde(rename = "regionName")]
    region_name: String,
}

fn get_local_ip() -> Result<String, io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;

    let local_addr = socket.local_addr()?;
    let local_ip = local_addr.ip();

    Ok(local_ip.to_string())
}

fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let url = "https://api.ipify.org?format=json";

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let response = client.get(url).send()?.error_for_status()?;
    let parsed_response = response.json::<PublicIpResponse>()?;

    Ok(parsed_response.ip)
}

fn get_ip_location(ip_address: &str) -> Result<IpLocation, Box<dyn Error>> {
    let url = format!("http://ip-api.com/json/{}", ip_address);

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let response = client.get(&url).send()?.error_for_status()?;
    let location_data = response.json::<IpLocation>()?;

    Ok(location_data)
}

pub fn ip() {
    let local_ip = get_local_ip().unwrap();
    let public_ip = get_public_ip().unwrap();
    let ip_location = get_ip_location(&public_ip).unwrap();

    println!("{}: {}", "Local IP".bright_green(), local_ip);
    println!(
        "{}: {} ({}, {})",
        "Public IP".bright_green(),
        public_ip,
        ip_location.region_name.blue(),
        ip_location.country.blue()
    );
}
