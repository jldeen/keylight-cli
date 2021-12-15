use clap::{App, Arg};

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    App::new("keylight")
        .version(&*version)
        .author("Jessica Deen <jessica.deen@microsoft.com>")
        .about("Easy CLI to control Elgato Keylight")
        .arg(
            Arg::with_name("ELGATO_IP")
                .long("elgato-ip-address")
                .short("ip")
                .help("Elgato Keylight IP address")
                .index(4)
                .required(true)
                .aliases(&["elgato_ip", "elgato-ip", "elgato ip"])
                .env("ELGATO_IP")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("NUMBER_OF_LIGHTS")
                .long("number-of-lights")
                .short("lights")
                .help("Number of Elgato Keylights in use")
                .required(true)
                .index(5)
                .aliases(&["number_of_lights", "number-of-lights", "number of lights"])
                .env("NUMBER_OF_LIGHTS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("SWITCH")
                .help("Switch value for light status: Accepted values are: on, off.")
                .required(true)
                .index(1)
                .default_value("off"),
        )
        .arg(
            Arg::with_name("BRIGHTNESS")
                .help("Brightness value for light: Accepted values are: low, medium, high.")
                .required(true)
                .index(2)
                .default_value("medium"),
        )
        .arg(
            Arg::with_name("TEMPERATURE")
                .help("Temperature value for light: Accepted values are: warm, medium, cool.")
                .required(true)
                .index(3)
                .default_value("medium"),
        )
}
