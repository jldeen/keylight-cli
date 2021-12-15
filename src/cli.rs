use clap::{App, Arg};

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    App::new("keylight")
        .version(&*version)
        .author("Jessica Deen <jessica.deen@microsoft.com>")
        .about("Easy CLI to control Elgato Keylight")
        .arg(
            Arg::with_name("switch")
                .index(1)
                .required(true)
                .value_name("on/off")
                .help("Switch value for light status: Accepted values are: on, off."),
        )
        .arg(
            Arg::with_name("brightness")
                .long("brightness")
                .short("b")
                .help("Brightness value for light: Accepted values are: low, medium, high.")
                .required(true)
                .env("brightness")
                .default_value("20"),
        )
        .arg(
            Arg::with_name("temperature")
                .long("temperature")
                .short("t")
                .help("Temperature value for light: Accepted values are: warm, medium, cool.")
                .required(true)
                .env("temperature")
                .default_value("213"),
        )
        .arg(
            Arg::with_name("elgato_ip")
                .long("elgato-ip")
                .short("i")
                .help("Elgato Keylight IP address")
                .required(true)
                .aliases(&["elgato_ip", "elgato-ip", "elgato ip"])
                .env("elgato_ip")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number_of_lights")
                .long("number-of-lights")
                .short("n")
                .help("Number of Elgato Keylights in use")
                .required(true)
                .aliases(&["number_of_lights", "number-of-lights", "number of lights"])
                .env("number_of_lights")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Verbose mode enabled")
        )
}
