#![allow(unused)]

use error_chain::error_chain;
use serde::Deserialize;
use serde_json::json;
use std::env;
use structopt::StructOpt;
use reqwest::Client;

// error_chain! {
//     foreign_links {
//         EnvVar(env::VarError);
//         HttpRequest(reqwest::Error);
//     }
// }

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Deserialize, Debug, StructOpt)]
struct Cli {
    /// Local IP address
    localaddress: String,
    /// Number of Lights
    numberoflights: u32,
    /// On/Off
    onoff: u32,
    /// Brightness
    brightness: u32,
    /// Temp
    temp: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let body = json!({
        "numberOfLights":&args.numberoflights,
        "lights":[
            {
                "on":&args.onoff,
                "brightness":&args.brightness,
                "temperature":&args.temp
            }
        ]
    });
    let url = format!("http://{}:{}", &args.localaddress, "9123/elgato/lights");
    
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
