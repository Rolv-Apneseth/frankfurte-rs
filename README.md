# Frankfurte-rs

[![Crates.io - Library](https://img.shields.io/crates/v/lib_frankfurter)](https://crates.io/crates/lib_frankfurter)
[![docs.rs - Library](https://img.shields.io/docsrs/lib_frankfurter)](https://docs.rs/lib_frankfurter)
[![AUR](https://img.shields.io/aur/version/frs)](https://aur.archlinux.org/packages/frs)

Rust library and CLI to interface with any Frankfurter API.
> *[Frankfurter](https://github.com/hakanensari/frankfurter) is a free, open source and [self-hostable](https://hub.docker.com/r/hakanensari/frankfurter) currency exchange rate API.
> It is based on [data sets](https://www.ecb.europa.eu/stats/policy_and_exchange_rates/euro_reference_exchange_rates/html/index.en.html) published by the European Central Bank.*

## Table of Contents

1. [About](#about)
2. [Library](#library)
3. [CLI](#cli)
1. [Installation](#installation)
    1. [Cargo](#cargo)
    2. [AUR](#aur)
2. [Usage](#usage)
3. [Self-hosting](#self-hosting)
4. [Related Projects](#related-projects)
5. [Credit](#credit)
6. [Licence](#licence)

## About

Frankfurte-rs (Frankfurters) is available as both an **executable** (`frs`) and a **Rust library**,
with the hope of providing safe and correct bindings for the API.

> [!NOTE]
> While the Frankfurter API works well most of the time, there are many edge cases and the returned
> errors are not exactly informative. If you run into any problems while using these bindings, please open
> an issue with the details of the error encountered.

## Library

Install with `cargo add lib_frankfurter` or simply add `lib_frankfurter` your `Cargo.toml`.

Then, check out the [this example](./lib/examples/basic.rs) to see basic usage.

## CLI

### Installation

#### Cargo

```bash
cargo install frankfurter_cli
```

#### AUR

```bash
paru -S frs
```

### Usage

```bash
# List the latest supported currencies
frs currencies
# Get the latest exchange rates, converting from the EUR
frs convert
# Get exchange rates from 01/01/2024 (or the closest available date with data), converting from the USD to PHP and NOK
frs convert USD -d 2024-01-01
# Get exchange rates over a time period from 01/01/2024 to the present date, converting from EUR to AUD
frs period EUR 2024-01-01 -t AUD
# Get exchange rates over a time period from 01/01/2024 to the 10/01/2024, converting from GBP to EUR and USD
frs period GBP -t EUR,USD 2024-01-01 2024-01-10
```

All options will print results in a table, but also accept the following options if you want the results in a different format:

- `--raw`: prints values only separated by tabs, useful for piping the data to different commands
- `--json`: prints the full `JSON` response from the server

View the full usage with `frs --help`.

### Self-hosting

A public, free-to-use version of the API is available [here](https://api.frankfurter.app/), and will be used by default. However, this repo comes with a [docker-compose.yml](./docker-compose.yml) for easy self-hosting of the `Frankfurter` API.

To set up and use a self-hosted version of the API, follow these steps:

1. Copy/clone the `docker-compose.yml` file to your system
2. Run `docker compose up -d --wait` to start up both the `postgresql` database and the `Frankfurter` API itself locally using Docker
3. When running commands, specify the desired API URL using either the `--url` flag or the `FRANKFURTER_URL` environment variable:
    - `frs --url http://localhost:8080`
    - `FRANKFURTER_URL="http://localhost:8080 frs`

## Related Projects

- [moneyman](https://github.com/sekunho/moneyman): Currency conversion using the same data sets from the ECB, but without the intermediary step of going through a separate API. Note that the caveat outlined there would also apply to `Frankfurter`.

## Credit

- [Frankfurter](https://github.com/hakanensari/frankfurter) of course, for be the underlying API that this project wraps
- [LanguageTool-Rust](https://github.com/jeertmans/languagetool-rust) for inspiration and a look at how Rust API bindings should look/function

## Licence  

[MIT](./LICENSE)
