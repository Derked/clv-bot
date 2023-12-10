use clv_bot::{arb_search, odds_api_client::OddsApiClient};
use serenity::all::ChannelId;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use tokio::time::{self, Duration};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let ctx = ctx.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60));
            let channel_id_str = env::var("CHANNEL_ID")
                .expect("No channel id found in dot env file")
                .parse::<u64>()
                .expect("Channel id is not a number");
            let channel_id = ChannelId::from(channel_id_str);
            let api_key = env::var("BET_ODDS_API_KEY").expect("No api key found in dot env file");
            let bookmakers = vec![
                "betrivers",
                "bovada",
                "betonlineag",
                "betmgm",
                "draftkings",
                "fanduel",
                "espnbet",
                "pinnacle",
                "superbook",
                "williamhill_us",
            ];
            let odds_api_client = OddsApiClient::new(&api_key, None);
            loop {
                interval.tick().await;
                let res = arb_search(&odds_api_client, &bookmakers).await;
                match res {
                    Ok(res) => {
                        channel_id
                            .say(&ctx.http, &format!("{}", res.to_string()))
                            .await
                            .expect("Error sending message");
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        });
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("No discord token found in dot env file");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
