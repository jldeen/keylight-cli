use clap::{Arg, Command};

pub fn get_app_cli(version: &str) -> Command<'_> {
    Command::new("keylight")
        .version(version)
        .author("Jessica Deen <jessicadeen@me.com>\nThompson Marzagao <thompson@marzagao.com>")
        .about("Easy CLI to control Elgato Keylight")
        .arg(
            Arg::new("switch")
                .index(1)
                .required(true)
                .value_name("on/off/status")
                .possible_values(["off", "on", "status"])
                .help("Toggle light on, off, or query current power state"),
        )
        .arg(
            Arg::new("brightness")
                .long("brightness")
                .short('b')
                .help("Brightness: percentage (0-100) or preset (low, medium, high)")
                .required(false)
                .env("brightness")
                .default_value("20"),
        )
        .arg(
            Arg::new("temperature")
                .long("temperature")
                .short('t')
                .help("Temperature: value (143-344) or preset (warm, medium, cool)")
                .required(false)
                .env("temperature")
                .default_value("213"),
        )
        .arg(
            Arg::new("elgato_ip")
                .long("elgato-ip")
                .short('i')
                .help("Elgato Keylight IP address")
                .required(false)
                .aliases(&["elgato_ip", "elgato-ip", "elgato ip"])
                .env("elgato_ip")
                .takes_value(true),
        )
        .arg(
            Arg::new("discover")
                .long("discover")
                .short('d')
                .help("Auto-discover Elgato Keylights on the local network via mDNS")
                .takes_value(false),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .help("Discovery timeout in seconds")
                .required(false)
                .default_value("10")
                .takes_value(true),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .multiple_occurrences(true)
                .help("Log Level"),
        )
}
