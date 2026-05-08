[![CI](https://github.com/marzagao/keylight-cli/actions/workflows/build-ci.yml/badge.svg)](https://github.com/marzagao/keylight-cli/actions/workflows/build-ci.yml)

# Elgato Keylight CLI

This is a cross platform lightweight CLI tool to simply and easily control your [Elgato Keylights](https://www.elgato.com/key-light) via local IP address. 

## Work To Be Done

- [X] ~~Support for `on` / `off` toggle arguements.~~ **Added in v0.2.0**
- [X] ~~Add help menu with `-h` flag.~~ **Added in v0.2.0**
- [X] ~~Add status to query for on/off~~ **Added in v0.2.2**
- [X] ~~Support for brightness and temperature via preset arguments, I.E, `low`, `medium`, and `high` or `warm`, `medium`, and `cool`.~~ **Added in v0.3.0**
- [X] ~~Support for brightness by percentage.~~ **Added in v0.3.0**
- [X] ~~Autodiscovery support.~~ **Added in v0.3.0**
- [X] ~~Testing with more than 1 Elgato Keylight.~~ **Tested in v0.3.0**

## Building The App

This app should build with minimal dependencies.  It's been tested with Rust 1.60+ on macOS and 2 Elgato Keylights.

```sh
cargo build --release
sudo cp target/release/keylight /usr/local/bin/keylight
```

## Running The App

There are two ways to target your Elgato Keylights: autodiscovery via mDNS or manual IP address.

### Autodiscovery (recommended)

Use the `--discover` / `-d` flag to automatically find all Elgato Keylights on your local network via mDNS:

```sh
# Check the status of all lights on the network
keylight status --discover

# Turn all discovered lights on
keylight on --discover

# Turn all discovered lights off
keylight off --discover
```

### Manual IP address

You can also specify the IP address directly:

```sh
keylight status --elgato-ip <ip-address-here>
keylight on --elgato-ip <ip-address-here> --brightness 30 --temperature 200
```

Environment variables can be provided in place of CLI arguments.

### Full usage

```
keylight v0.3.0
Jessica Deen <jessicadeen@me.com>, Thompson Marzagao <thompson@marzagao.com>
Easy CLI to control Elgato Keylight

USAGE:
    keylight [OPTIONS] <on/off/status>

ARGS:
    <on/off/status>    Toggle light on, off, or query current power state [possible values: off,
                       on, status]

OPTIONS:
    -b, --brightness <brightness>
            Brightness: percentage (0-100) or preset (low, medium, high) [env: brightness=] [default: 20]

    -d, --discover
            Auto-discover Elgato Keylights on the local network via mDNS

    -h, --help
            Print help information

    -i, --elgato-ip <elgato_ip>
            Elgato Keylight IP address [env: elgato_ip=]

    -t, --temperature <temperature>
            Temperature: value (143-344) or preset (warm, medium, cool) [env: temperature=] [default: 213]

        --timeout <timeout>
            Discovery timeout in seconds [default: 5]

    -v, --verbose
            Log Level

    -V, --version
            Print version information
```

## Scheduling

You can use cron to automatically turn your lights on and off on a schedule. For example, to turn lights on at 8am and off at 7pm on weekdays:

```sh
crontab -e
```

Add the following lines:

```
0 8 * * 1-5 /usr/local/bin/keylight on --discover
0 19 * * 1-5 /usr/local/bin/keylight off --discover
```

## Setting up as daemon on macOS

The daemon watches your camera state and automatically turns your Elgato Keylights on/off using autodiscovery.

```sh
# clone this repo
# from within root of this repo folder
mkdir -p ~/bin && cp onair.sh ~/bin

# add your system username to the plist file
sed -i '' 's/<REPLACE_USER>/your-username-here/g' com.keylight.daemon.plist

# copy updated plist to launchdaemon folder
cp com.keylight.daemon.plist /Library/LaunchDaemons/com.keylight.daemon.plist

# load/start daemon
sudo launchctl load -w /Library/LaunchDaemons/com.keylight.daemon.plist

# view logs
tail -f /tmp/keylight.stdout
tail -f /tmp/keylight.stderr
```
