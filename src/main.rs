#![allow(unused)]

use structopt::StructOpt;
use std::io::{stdout, Write};
use curl::easy::Easy;
use std::env;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Local IP address
    localaddress: String,
    // /// Number of Lights
    // numoflights: String,
    // /// On/Off
    // onoff: String,
    // /// Brightness
    // brightness: String,
    // /// Temp
    // temp: String,
}

fn main() {
    let args = Cli::from_args();

    let url = format!("http://{}:{}", &args.localaddress, "9123/elgato/lights");
    
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
}
