mod cli;

// Logging
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

// Required deps
use cli::get_app_cli;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verbose;

    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let matches = get_app_cli(&version).get_matches();

    match matches.occurrences_of("verbose") {
        0 => verbose = LevelFilter::Off,
        1 => verbose = LevelFilter::Info,
        2 => verbose = LevelFilter::Debug,
        3 | _ => verbose = LevelFilter::Trace,
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

    let switch = if matches.value_of("switch").unwrap() == "off" {
        0;
        let power_status = "off";
        println!("Elgato Keylight is: {}", power_status);
    } else {
        1;
        let power_status = "on";
        log::info!("Elgato Keylight is: {}", power_status);
    };

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
    log::info!("State: {}", url);

    let client = Client::new();

    let response = client.put(url).json(&body).send().await?;

    let status = response.status();
    let response_body = response.text().await?;
    let response_json: serde_json::Value = serde_json::from_str(&response_body)?;

    log::info!("Response status: {}", status);
    log::info!("Response text: {}", response_body);
    log::info!("Response json: {:?}", response_json);

    Ok(())
}
