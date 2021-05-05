use super::message;
use serenity::futures::TryStreamExt;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use sqlx::Row;

pub struct Handler {
    pool: sqlx::SqlitePool,
    parser: message::Parser,
}

impl Handler {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        let parser = message::Parser::new();
        Handler { pool, parser }
    }
}

async fn send_msg(ctx: &Context, msg: Message, data: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, data).await {
        println!("Error sending message: {:?}", why);
    }
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if ctx.cache.current_user().await.id == msg.author.id {
            return;
        }
        let guild_id = if let Some(g) = msg.guild(ctx.cache.clone()).await.map(|g| *g.id.as_u64()) {
            g
        } else {
            send_msg(&ctx, msg, "I cannot be used in a private message").await;
            return;
        };
        let replaced_message = msg.content_safe(ctx.cache.clone()).await;
        let response = match self.parser.parse(&replaced_message) {
            Some(message::Message::Ping) => Some("!Pong".to_owned()),
            Some(message::Message::Incr(add_re_matches)) => {
                for c in add_re_matches {
                    println!("incremented {}", c);
                    let i = sqlx::query("INSERT INTO karma (name, guild_id, karma) VALUES($1, $2, 1) ON CONFLICT(name, guild_id) DO UPDATE SET karma=karma + 1;").bind(c).bind(guild_id.to_string()).execute(&self.pool).await;
                    match i {
                        Err(e) => {
                            println!("Updating karma failed with: {}", &e.to_string());
                        }
                        _ => {}
                    }
                }
                None
            }
            Some(message::Message::Decr(add_re_matches)) => {
                for c in add_re_matches {
                    println!("deccremented {}", c);
                    let i = sqlx::query("INSERT INTO karma (name, guild_id, karma) VALUES($1, $2, -1) ON CONFLICT(name, guild_id) DO UPDATE SET karma=karma - 1;").bind(c).bind(guild_id.to_string()).execute(&self.pool).await;
                    match i {
                        Err(e) => {
                            println!("Updating karma failed with: {}", &e.to_string());
                        }
                        _ => {}
                    }
                }
                None
            }
            Some(message::Message::Karma(name)) => {
                let mut rows = sqlx::query(
                    "SELECT name,karma,guild_id FROM karma WHERE name=$1 AND guild_id=$2",
                )
                .bind(name)
                .bind(guild_id.to_string())
                .fetch(&self.pool);
                let s_row = rows.try_next().await;
                if let Ok(Some(row)) = s_row {
                    Some(format!(
                        "{} has karma {}",
                        row.try_get::<&str, _>("name").unwrap(),
                        row.try_get::<i64, _>("karma").unwrap(),
                    ))
                } else {
                    None
                }
            }
            None => None,
        };
        if let Some(s) = response {
            send_msg(&ctx, msg, &s).await;
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
