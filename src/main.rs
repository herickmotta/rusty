mod commands;

use dotenv::dotenv;
use std::env;

use songbird::SerenityInit;

use serenity::{
    async_trait,
    client::{Client, EventHandler, Context},
    framework::{
        standard::macros::group,
        StandardFramework,
    },
    model::gateway::Ready,
    prelude::GatewayIntents,
};

use crate::commands::player::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(join, leave, mute, play, ping, unmute)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
      .configure(|c| c.prefix("!"))
      .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
      .event_handler(Handler)
      .framework(framework)
      .register_songbird()
      .await
      .expect("Err creating client");

    tokio::spawn(async move {
      let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}


