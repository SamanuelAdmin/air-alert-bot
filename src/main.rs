use tokio::time::{sleep, Duration};
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use teloxide::prelude::*;

// local modules
mod parser;
use parser::{Parser, Alert};


const TRACKED_REGIONS: [u32; 13] = [
    179965,
    179964,
    179897,
    180071,
    179960,
    179766,
    179767,
    179738,
    180093,
    180096,
    180097,
    180098,
    180099,
];



struct EnvData {
    alerts_api_token: String,
    bot_api_token: String
}


fn get_env_data() -> Result<EnvData, Box<dyn std::error::Error>>{
    // loading dotenv file to the env
    dotenv().ok();

    // getting env variables
    Ok(EnvData {
        alerts_api_token: env::var("ALERTS_API_TOKEN")?,
        bot_api_token: env::var("BOT_API_TOKEN")?
    })
}


// This is needed to run `async` main function via tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_data = get_env_data()?;
    
    let mut parser = Parser::new(
        "https://api.alerts.in.ua/v1/alerts/active.json".to_string(),
        env_data.alerts_api_token, &TRACKED_REGIONS
    );
    let bot = Bot::new(env_data.bot_api_token).auto_send();
    let mut chat_list: Vec<ChatId> = Vec::new();

    // for test only!!!
    chat_list.push(ChatId(1178323450));

    loop {
        let changed_alerts: Vec<&Alert> = parser.parse().await?;

        for changed_alert in changed_alerts {
            for chat_id in &chat_list {
                bot.send_message(
                    *chat_id, format!("{}", changed_alert)
                ).await?;
            }
            println!("{}", changed_alert);
        }
        

        // blocking for 10 seconds
        sleep(Duration::from_millis(10000)).await;
    }

    Ok(())
}
