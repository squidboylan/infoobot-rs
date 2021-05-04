use sqlx::Connection;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;

use serenity::prelude::*;

mod handler;

#[tokio::main]
async fn main() {
    let db = env::var("DATABASE_URL").expect("Please set DATABASE_URL");
    let pool = sqlx::SqlitePool::connect(&db).await.unwrap();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Please set DISCORD_TOKEN");
    let username = env::var("IMGFLIP_USERNAME").expect("Please set IMGFLIP_USERNAME");
    let password = env::var("IMGFLIP_PASSWORD").expect("Please set IMGFLIP_PASSWORD");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(handler::Handler { pool })
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
