use serde::Deserialize;
use serde_json::Value;
use serenity::{model::prelude::*, prelude::*, Client};
use std::fs;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        unimplemented!();
    }
}

#[derive(Deserialize)]
struct Config {
    client_token: String,
    facts: Vec<String>,
    victims: Vec<String>,
}

fn read_config(path: &str) -> Config {
    let contents = fs::read_to_string(path).expect("[!] Error reading config file!");
    let config: Config = serde_json::from_str(&contents).expect("[!] Error parsing JSON config!");
    return config;
}

fn main() {
    let config = read_config("config.json");
    let mut client =
        Client::new(config.client_token, Handler).expect("Couldn't create the new client!");
}
