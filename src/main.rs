#![allow(unused)]
mod cli;

use cli::get_app_cli;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));

    let matches = get_app_cli(&version).get_matches();
    let elgato_ip = matches.value_of("ELGATO_IP").unwrap();
    let numberoflights = matches.value_of("NUMBER_OF_LIGHTS").unwrap();
    let switch = matches
        .value_of("switch")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap();
    let brightness = matches
        .value_of("brightness")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap();
    let temperature = matches
        .value_of("temperature")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap();

    println!("Value for switch: {}", switch);

    let body = json!({
        "numberOfLights":numberoflights,
        "lights":[
            {
                "on":switch,
                "brightness":brightness,
                "temperature":temperature
            }
        ]
    });

    let url = format!("http://{}:{}", elgato_ip, "9123/elgato/lights");

    println!("State: {}", url);

    let client = reqwest::Client::new();

    let response = client.put(url).json(&body).send().await?;

    let status = response.status();
    let response_body = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_body)?;

    println!("Response status: {}", status);
    println!("Response text: {}", response_body);
    println!("Response json: {:?}", response_json);

    Ok(())
}
