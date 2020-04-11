extern crate clap;

use serde::Deserialize;
mod cli;
mod app;

#[macro_use] extern crate log;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Configuration {
    bot_token: String,
    username: String,
    password: String,
    url: String,
    port: String,
    allowed_users: Vec<String>
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            bot_token: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            url: "".to_string(),
            port: "9091".to_string(),
            allowed_users: vec![]
        }
    }
}

fn main() {
    env_logger::init();
    let mut args: Configuration = Configuration::default();
    cli::arg_parse(&mut args);
    app::run(&args);
}



