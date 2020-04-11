Transmission Telegram Bot
=========================

A simple Telegram chat bot service that can be registered with the BotFather
for interacting with a Transmission BitTorrent client to manage torrents.

This project is a Rust learning experience for me, and as you might expect, is
provided with no warranty, liabilities, etc. The user is entirely liable for
anything downloaded with this tool.

## Usage

The CLI can be givne arguments in two ways:

* As CLI arguments
* within a TOML file, located at `$HOME/.transmission-telegram-bot.toml`

You can also use the `--config` option to provide a path to a TOML file. See the
`--help` command for more details.

## Building

The project can be built using Cargo. Simply run `cargo build` from within the
project directory, or run `cargo run` to build and run it locally in debug
mode.
