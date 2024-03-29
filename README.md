[![CI](https://github.com/jldeen/keylight-cli/actions/workflows/build-ci.yml/badge.svg)](https://github.com/jldeen/keylight-cli/actions/workflows/build-ci.yml)

# Elgato Keylight CLI

This is a cross platform lightweight CLI tool to simply and easily control your [Elgato Keylights](https://www.elgato.com/key-light) via local IP address. 

## Work To Be Done

- [X] ~~Support for `on` / `off` toggle arguements.~~ **Added in v0.2.0**
- [X] ~~Add help menu with `-h` flag.~~ **Added in v0.2.0**
- [X] ~~Add status to query for on/off~~ **Added in v0.2.2**
- [ ] Support for brightness and temperature via preset arguments, I.E, `low`, `medium`, and `high` or `warm`, `medium`, and `cool`.
- [ ] Support for brightness by percentage.
- [ ] Autodiscovery support.
- [ ] Testing with more than 1 Elgato Keylight.

## Building The App

This app should build with minimal dependencies.  It's been tested with Rust 1.60 on macOS Sonoma 14.4.1 and 1 Elgato Keylight.

`cargo build`

## Running The App

This CLI tool has three mandatory parameters and two optional ones (that have default values).  There are environment variables that can be provided in place of CLI arguments.

```
keylight v0.2.2
Jessica Deen <jessicadeen@me.com>
Easy CLI to control Elgato Keylight

USAGE:
    keylight [OPTIONS] --elgato-ip <elgato_ip> --number-of-lights <number_of_lights> <on/off/status>

ARGS:
    <on/off/status>    Toggle light on, off, or query current power state [possible values: off,
                       on, status]

OPTIONS:
    -b, --brightness <brightness>
            Brightness value for light [env: brightness=] [default: 20]

    -h, --help
            Print help information

    -i, --elgato-ip <elgato_ip>
            Elgato Keylight IP address [env: elgato_ip=192.168.184.166]

    -n, --number-of-lights <number_of_lights>
            Number of Elgato Keylights in use [env: number_of_lights=1]

    -t, --temperature <temperature>
            Temperature value for light [env: temperature=] [default: 213]

    -v, --verbose
            Log Level

    -V, --version
            Print version information
```

## Setting up as daemon on macOS

```sh
# clone this repo
# from within root of this repo folder
mkdir -p ~/bin && cp onair.sh ~/bin # makes a bin directory in your user's home folder, copies onair script to that folder

# add your ip address and system username to the plist file
sed -i 's/<REPLACE_IP_ADDRESS>/your-elgato-ip-address-here/g' com.keylight.task.plist
sed -i 's/<REPLACE_USER>/your-username-here/g' com.keylight.task.plist 

# copy updated plist to launchdaemon folder
cp com.keylight.task.plist /Library/LaunchDaemons/com.keylight.task.plist

# change permissions to plist file
sudo chown root:wheel /Library/LaunchDaemons/com.keylight.task.plist

# load/start daemon/plist
sudo launchctl unload -w /Library/LaunchDaemons/com.keylight.task.plist
sudo launchctl load -w /Library/LaunchDaemons/com.keylight.task.plist
sudo launchctl start -w /Library/LaunchDaemons/com.keylight.task.plist

# view logs
tail -f /tmp/keylight.stdout #standard out logs
tail -f /tmp/keylight.stderr #standard error logs
```