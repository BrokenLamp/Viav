extern crate log;
extern crate pretty_env_logger;

use core::time::Duration;
use dotenv::dotenv;
use log::{info, trace};
use serenity::client::{bridge::gateway::GatewayIntents, Client};
use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use sqlx::postgres::PgPool;
use std::env;

mod channel_utils;
mod commands;
mod deck;
mod handler;
mod help;
mod voice_create;
mod voice_destroy;
mod voice_events;

use commands::GENERAL_GROUP;
use handler::Handler;

pub const MASTER_USER: UserId = UserId(222554302793646083);
pub const THREADS: usize = 512;

#[tokio::main(core_threads = 64)]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();

    println!(include_str!("terminal_start.txt"));

    info!("Connecting to Postgres");
    let pool = PgPool::builder()
        .max_size(8)
        .build(&env::var("DATABASE_URL").expect("database url"))
        .await
        .unwrap();
    info!("Connected.");

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"))
        .event_handler(Handler)
        .cache_update_timeout(Duration::from_secs(10))
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_VOICE_STATES
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::GUILD_INTEGRATIONS
                | GatewayIntents::GUILD_PRESENCES
                | GatewayIntents::GUILD_EMOJIS,
        )
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix("-viav "))
                .group(&GENERAL_GROUP),
        )
        .await
        .expect("Error creating client");

    let num_shards = env::var("NUM_SHARDS")
        .ok()
        .and_then(|x| x.parse::<u64>().ok())
        .unwrap_or(1u64);

    println!("Shards: {}", num_shards);

    trace!("Starting Viav");

    if let Err(why) = client.start_shards(num_shards).await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
