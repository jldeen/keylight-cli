[![CI](https://github.com/jldeen/keylight-cli/actions/workflows/build-ci.yml/badge.svg)](https://github.com/jldeen/keylight-cli/actions/workflows/build-ci.yml)

# Elgato Keylight CLI

This is a cross platform lightweight CLI tool to simply and easily control your [Elgato Keylights](https://www.elgato.com/key-light) via local IP address. 

## Work To Be Done

- [X] ~~Support for `on` / `off` toggle arguements. **Added in v0.2.0**~~
- [X] ~~Add help menu with `-h` flag. **Added in v0.2.0**~~
- [ ] Support for brightness and temperature via preset arguments, I.E, `low`, `medium`, and `high` or `warm`, `medium`, and `cool`.
- [ ] Testing with more than 1 Elgato Keylight.

## Building The App

This app should build with minimal dependencies.  It's been tested with Rust 1.56 on macOS Monterey 12.1 and 1 Elgato Keylight.

`cargo build`

## Running The App

This CLI tool has three mandatory parameters and two optional ones (that have default values).  There are environment variables that can be provided in place of CLI arguments.

```
keylight cli v0.2.0
Jessica Deen <jessica.deen@microsoft.com>
A Simple lightweight CLI to control Elgato Keylights from the command line

USAGE:
    keylight [FLAGS] --elgato-ip <Local Elgato IP Address> --number-of-lights <Number of Elgato Keylights> --brightness <Brightness> --temperature <Temperature> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Resource group location for the ContainerApps environment.

OPTIONS:
    -i, --elgato-ip <Local Elgato IP Address>
            Local Elgato IP Address [env: elgato_ip=]

    -n, --number-of-lights <Number of Elgato Keylights>
            Number of Elgato Keylights to control [env: number_of_lights=]

    -b, --brightness <Brightness>
            Brightness level [env: brightness=] [default: 20]

    -t, --temperature <Temperature>
            Temperature level [env: temperature=] [default: 213]


ARGS:
    <on/off>    Toggle switch value to turn keylight(s) on or off.
```
