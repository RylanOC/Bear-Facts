use serenity::{model::prelude::*, prelude::*, Client};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        unimplemented!();
    }
}

fn main() {
    let mut client = Client::new("<token>", Handler).expect("Couldn't create the new client!");
}
