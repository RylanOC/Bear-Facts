use serde_json::Value;
use serenity::{model::prelude::*, prelude::*, Client};
use std::fs;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        unimplemented!();
    }
}

fn read_config(path: &str) -> Value {
    let contents = fs::read_to_string(path).expect("[!] Error reading config file!");
    let config = serde_json::from_str(&contents).expect("[!] Error parsing JSON config!");
    return config;
}

fn main() {
    let mut client = Client::new("<token>", Handler).expect("Couldn't create the new client!");
}
