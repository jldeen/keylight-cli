use clap::{App, Arg};

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    App::new("keylight")
        .version(&*version)
        .author("Jessica Deen <jessica.deen@microsoft.com>")
        .about("Easy CLI to control Elgato Keylight")
        .arg(
            Arg::with_name("SWITCH")
                .help("Switch value for light status: Accepted values are: on, off.")
                .index(1)
                .default_value("off"),
        )
}