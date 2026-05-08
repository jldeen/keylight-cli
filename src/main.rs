mod cli;
mod discovery;

// Logging
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::time::Duration;

// Required deps
use cli::get_app_cli;
use discovery::discover_lights;
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

    // Determine target lights
    let discover = matches.is_present("discover");
    let elgato_ip = matches.value_of("elgato_ip");

    let targets: Vec<(String, u16, Option<String>)> = if discover {
        println!("Discovering Elgato Keylights on the network...");
        let timeout_secs: u64 = matches
            .value_of("timeout")
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);
        let lights = discover_lights(Duration::from_secs(timeout_secs))?;
        if lights.is_empty() {
            eprintln!("Error: No Elgato Keylights found on the network.");
            std::process::exit(1);
        }
        for light in &lights {
            println!("  Found: {} ({}:{})", light.name, light.ip, light.port);
        }
        lights
            .into_iter()
            .map(|l| (l.ip, l.port, Some(l.name)))
            .collect()
    } else if let Some(ip) = elgato_ip {
        vec![(ip.to_string(), 9123, None)]
    } else {
        eprintln!("Error: Either --discover or --elgato-ip must be specified.");
        std::process::exit(1);
    };

    let switch = match matches.value_of("switch").unwrap() {
        "off" => 0,
        "on" => 1,
        "status" => 2,
        _ => 0,
    };

    if switch == 0 {
        println!("Turning {} Elgato Keylight(s) off...", targets.len());
    } else if switch == 1 {
        println!("Turning {} Elgato Keylight(s) on...", targets.len());
    }

    let has_brightness = matches.occurrences_of("brightness") > 0;
    let has_temperature = matches.occurrences_of("temperature") > 0;

    let brightness: Option<u8> = if has_brightness {
        let brightness_str = matches.value_of("brightness").unwrap();
        Some(match brightness_str {
            "low" => 10,
            "medium" => 50,
            "high" => 100,
            s => {
                let s = s.strip_suffix('%').unwrap_or(s);
                match s.parse::<u8>() {
                    Ok(v) if v <= 100 => v,
                    _ => {
                        eprintln!("Error: Brightness must be 0-100 or a preset (low, medium, high).");
                        std::process::exit(1);
                    }
                }
            }
        })
    } else {
        None
    };

    let temperature: Option<f32> = if has_temperature {
        let temperature_str = matches.value_of("temperature").unwrap();
        Some(match temperature_str {
            "warm" => 344.0,
            "medium" => 213.0,
            "cool" => 143.0,
            s => match s.parse::<f32>() {
                Ok(v) => v,
                _ => {
                    eprintln!("Error: Temperature must be a number (143-344) or a preset (warm, medium, cool).");
                    std::process::exit(1);
                }
            },
        })
    } else {
        None
    };

    let mut body = json!({
        "numberOfLights":1,
        "lights":[
            {
                "on":switch
            }
        ]
    });
    if let Some(b) = brightness {
        body["lights"][0]["brightness"] = json!(b);
    }
    if let Some(t) = temperature {
        body["lights"][0]["temperature"] = json!(t);
    }

    let client = Client::new();
    let mut errors = Vec::new();

    for (ip, port, name) in &targets {
        let url = format!("http://{}:{}/elgato/lights", ip, port);
        log::info!("Sending request to: {}", url);

        let label = match name {
            Some(n) => format!("{} ({})", n, ip),
            None => ip.to_string(),
        };

        let result: Result<(), Box<dyn std::error::Error>> = async {
            if switch == 2 {
                // GET to read current settings without modifying them
                let response = client.get(&url).send().await?;
                log::info!("Response status: {}", response.status());

                let response_body = response.text().await?;
                log::info!("Response text: {}", response_body);

                let v: Value = serde_json::from_str(&response_body)?;
                let light = &v["lights"][0];

                let power = if light["on"] == 1 { "on" } else { "off" };
                let brightness = &light["brightness"];
                let temperature = &light["temperature"];

                println!("{}:", label);
                println!("  Power:       {}", power);
                println!("  Brightness:  {}%", brightness);
                let temp_val = temperature.as_f64().unwrap_or(0.0);
                let kelvin = if temp_val > 0.0 {
                    (1_000_000.0 / temp_val) as u32
                } else {
                    0
                };
                println!("  Temperature: {} (~{}K)", temperature, kelvin);
            } else {
                // PUT to change settings
                let response = client.put(&url).json(&body).send().await?;

                let response_success = response.status();
                log::info!("Response status: {}", response_success);

                let response_body = response.text().await?;
                log::info!("Response text: {}", response_body);

                let response_json: serde_json::Value = serde_json::from_str(&response_body)?;
                log::info!("Response json: {:?}", response_json);
            }
            Ok(())
        }
        .await;

        if let Err(e) = result {
            eprintln!("Error communicating with light at {}:{}: {}", ip, port, e);
            errors.push(format!("{}:{}", ip, port));
        }
    }

    if !errors.is_empty() {
        eprintln!("Failed to reach {} light(s): {}", errors.len(), errors.join(", "));
        std::process::exit(1);
    }

    Ok(())
}
