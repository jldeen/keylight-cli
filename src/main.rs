mod cli;

// Logging
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

// Required deps
use cli::get_app_cli;
use reqwest::Client;
use serde_json::{json, Value};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let matches = get_app_cli(&version).get_matches();

    let verbose = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(verbose)
        .parse_env("LOG_LEVEL")
        .init();

    let elgato_ip = matches.value_of("elgato_ip").unwrap();
    let numberoflights = matches.value_of("number_of_lights").unwrap();

    let switch = match matches.value_of("switch").unwrap() {
        "off" => 0,
        "on" => 1,
        "status" => 2,
        _ => 0,
    };

    // status
    if switch == 0 {
        let power_status = "off";
        println!("Elgato Keylight is: {}", power_status);
    } else if switch == 1 {
        let power_status = "on";
        println!("Elgato Keylight is: {}", power_status);
    }

    let brightness = matches
        .value_of("brightness")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap();

    let temperature = matches
        .value_of("temperature")
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap();

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
    log::info!("status: {}", url);

    let client = Client::new();

    let response = client.put(url).json(&body).send().await?;

    // response status code
    let response_success = response.status();
    log::info!("Response status: {}", response_success);

    // body
    let response_body = response.text().await?;
    log::info!("Response text: {}", response_body);

    // Json data
    let response_json: serde_json::Value = serde_json::from_str(&response_body)?;
    log::info!("Response json: {:?}", response_json);

    if switch == 2 {
        let v: Value = serde_json::from_str(&response_body)?;

        let query = &v["lights"][0]["on"];

        if query == 0 {
            let status = "off";
            println!("Elgato light is: {}", status);
        } else if query == 1 {
            let status = "on";
            println!("Elgato light is: {}", status);
        }
    }

    Ok(())
}
