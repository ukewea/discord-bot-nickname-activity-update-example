use serenity::all::ActivityData;
use serenity::all::GuildId;
use serenity::all::GuildInfo;
use serenity::all::GuildPagination;
use serenity::http;
use serenity::prelude::*;
use tracing::error;
use tracing::trace;
use tracing::warn;
use tracing::debug;
use std::env;
use tracing::Level;
use tracing_subscriber;

async fn test_get_guilds(http_client: &http::Http) -> Result<Vec<GuildInfo>, serenity::Error> {
    let mut guilds = Vec::<GuildInfo>::new();
    let mut retry_count = 3;
    let mut last_id: Option<GuildId> = None;
    let limit = 100;

    loop {
        let target = last_id.map(GuildPagination::After);

        match http_client.get_guilds(target, Some(limit)).await {
            Ok(guilds_partial) => {
                let partial_len = guilds_partial.len();
                guilds.extend(guilds_partial);
                if partial_len >= limit as usize {
                    trace!("Got {} guilds, continue", limit);
                    last_id = Some(guilds.last().unwrap().id);
                } else {
                    trace!("Got all guilds, break");
                    break;
                }
            }
            Err(why) => {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                retry_count -= 1;
                if retry_count == 0 {
                    return Err(why);
                }
                continue;
            }
        };
    }

    Ok(guilds)
}

#[tokio::main]
async fn main() {
    macro_rules! sleep_then_continue {
        () => {
            debug!("Sleeping for 15 seconds before continuing");
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
            continue;
        };
    }

    tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::default();
    let mut client = Client::builder(&token, intents)
        .await
        .expect("Err creating client");
    let shard_manager = client.shard_manager.clone();
    let http_client = client.http.clone();

    tokio::spawn(async move {
        // Start two shards. Note that there is an ~5 second ratelimit period between when one shard
        // can start after another.
        if let Err(why) = client.start_shards(1).await {
            error!("Client error: {why:?}");
        }
    });

    loop {
        let guilds = match test_get_guilds(&*http_client).await {
            Ok(guilds) => guilds,
            Err(why) => {
                warn!("Error getting guilds: {why:?}");
                sleep_then_continue!();
            }
        };

        debug!("Update nicknames in guilds:");
        for g in &guilds {
            let current_date = &chrono::Utc::now().to_rfc3339()[5..19];
            match http_client
                .edit_nickname(g.id, Some(current_date), None)
                .await
            {
                Ok(_) => debug!(
                    "  Updated nickname for guild {} to {}",
                    g.name, current_date
                ),
                Err(why) => {
                    warn!("  Error updating nickname for guild {}: {why:?}", g.name);
                    sleep_then_continue!();
                }
            };
        }

        let shard_runners = shard_manager.runners.lock().await;
        for (shard_id, runner) in shard_runners.iter() {
            let current_date = &chrono::Utc::now().to_rfc3339()[5..19];
            let new_activity = ActivityData::custom(current_date);
            runner.runner_tx.set_activity(Some(new_activity));
            debug!(
                "Updated activity for shard {} to {}",
                shard_id, current_date
            );
        }

        sleep_then_continue!();
    }
}
