use clap::{Command, Arg};

pub fn get_app_cli<'a>(version: &'a str) -> Command<'a,> {
    Command::new("keylight")
        .version(&*version)
        .author("Jessica Deen <jessica.deen@microsoft.com>")
        .about("Easy CLI to control Elgato Keylight")
        .arg(
            Arg::new("switch")
                .index(1)
                .required(true)
                .value_name("on/off")
                .possible_values(&["off", "on"])
                .help("Toggle light on or off"),
        )
        .arg(
            Arg::new("brightness")
                .long("brightness")
                .short('b')
                // .possible_values(&["low", "medium", "high"])
                .help("Brightness value for light")
                .required(false)
                .env("brightness")
                .default_value("20"),
        )
        .arg(
            Arg::new("temperature")
                .long("temperature")
                .short('t')
                // .possible_values(&["warm", "medium", "cool"])
                .help("Temperature value for light")
                .required(false)
                .env("temperature")
                .default_value("213"),
        )
        .arg(
            Arg::new("elgato_ip")
                .long("elgato-ip")
                .short('i')
                .help("Elgato Keylight IP address")
                .required(true)
                .aliases(&["elgato_ip", "elgato-ip", "elgato ip"])
                .env("elgato_ip")
                .takes_value(true),
        )
        .arg(
            Arg::new("number_of_lights")
                .long("number-of-lights")
                .short('n')
                .help("Number of Elgato Keylights in use")
                .required(true)
                .aliases(&["number_of_lights", "number-of-lights", "number of lights"])
                .env("number_of_lights")
                .takes_value(true),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .multiple_occurrences(true)
                .help("Log Level")
        )
}
