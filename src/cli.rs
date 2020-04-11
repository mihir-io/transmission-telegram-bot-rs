use crate::Configuration;
use clap::{App, Arg};
use std::fs;
use log::{warn, error};
use dirs::home_dir;
use std::path::{PathBuf};
use std::env::current_dir;

pub(crate) fn arg_parse(args: &mut Configuration){

    // Generate a path to the .telegram-transmission-bot.toml config
    // file. If the user has no home path, use the current directory.
    let mut config_path = match home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(current_dir().unwrap())
    };

    config_path.push(".telegram-transmission-bot");
    config_path.set_extension("toml");

    let config_string = config_path.to_str().unwrap();

    let matches = App::new("telegram-transmission-bot")
        .arg(Arg::with_name("config")
            .help("Path to config file on disk")
            .takes_value(true)
            .default_value(config_string)
            .long("config"))
        .arg(Arg::with_name("bot_token")
            .help("Telegram bot API token")
            .takes_value(true)
            .long("bot-token"))
        .arg(Arg::with_name("username")
            .help("username for Transmission remote server (only use if auth is enabled)")
            .takes_value(true)
            .long("username"))
        .arg(Arg::with_name("password")
            .help("password for Transmission remote server (only use if auth is enabled)")
            .takes_value(true)
            .long("password"))
        .arg(Arg::with_name("url")
            .help("URL for Transmission remote server")
            .takes_value(true)
            .long("url"))
        .arg(Arg::with_name("allowed_users")
            .help("Telegram username of user authorized to access this bot instance (defaults to no one)")
            .takes_value(true)
            .multiple(true)
            .long("allowed-user"))
        .arg(Arg::with_name("use_https")
            .help("Use HTTPS for Transmission REST calls")
            .takes_value(true)
            .default_value("true")
            .long("use-https"))
        .get_matches();

    info!("Reading configuration TOML at {}", matches.value_of("config").unwrap().to_string());
    let config_toml: String = fs::read_to_string(
        matches.value_of("config").unwrap_or_default().to_string()
    ).unwrap_or_default();
    let toml_data: Configuration = toml::from_str(config_toml.as_str()).unwrap_or_else(
        |arg| -> Configuration {
            warn!("Could not load TOML file: {}", arg);
            return Configuration::default()
        }
    );

    args.bot_token = matches.value_of("bot_token").unwrap_or_else(|| toml_data.bot_token.as_str()).to_string();
    args.username = matches.value_of("username").unwrap_or_else(|| toml_data.username.as_str()).to_string();
    args.password = matches.value_of("password").unwrap_or_else(|| toml_data.password.as_str()).to_string();
    args.url = matches.value_of("url").unwrap_or_else(|| toml_data.url.as_str()).to_string();
    args.port = matches.value_of("port").unwrap_or_else(|| toml_data.port.as_str()).to_string();

    // Append CLI allowed users to args struct. If none are provided in CLI args, default to an empty list.
    for username in matches.values_of("allowed_users").unwrap_or_default(){
        args.allowed_users.push(username.to_string());
    }

    // Handle the case where the CLI args don't have allowed users, but the TOML config might.
    if args.allowed_users.is_empty(){
        for user in toml_data.allowed_users {
            args.allowed_users.push(user);
        }
    }

    validate_args(args);
}

fn validate_args(args: &Configuration){
    if args.bot_token.is_empty() {
        error!("A Telegram bot token must be provided with the --bot-token argument. Run this tool with --help for more info.");
        std::process::exit(1);
    }
    if args.url.is_empty() {
        error!("A URL to the Transmission RPC server must be provided with the --url argument. Run this tool with --help for more info.");
        std::process::exit(1);
    }

    if args.allowed_users.is_empty() {
        warn!("No allowed users were provided with the --allowed-users argument. All commands to the bot will be ignored.");
    }

    if args.username.is_empty() || args.password.is_empty() {
        info!("Credentials were not provided with --username and --password for the Transmission RPC server. Server auth will be done anonymously.")
    }
}