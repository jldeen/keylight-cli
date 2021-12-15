#![allow(unused)]
mod cli;

use cli::get_app_cli;
use serde_json::json;
use std::env;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = format!(
        "{}.{}",
        env!("CARGO_PKG_VERSION"),
        option_env!("BUILD_BUILDID").unwrap_or("0")
    );
    
    let matches = get_app_cli(&version).get_matches();
    let elgato_ip = matches.value_of("ELGATO_IP").unwrap();
    let numberoflights = matches.value_of("NUMBER_OF_LIGHTS").unwrap();

    let switch = matches.value_of("SWITCH").unwrap();
    let brightness = matches.value_of("BRIGHTNESS").unwrap();
    let temperature = matches.value_of("TEMPERATURE").unwrap();

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

    let response = client
        .put(url)
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    let response_body = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_body)?;

    println!("Response status: {}", status);
    println!("Response text: {}", response_body);
    println!("Response json: {:?}", response_json); 

    Ok(())
}
