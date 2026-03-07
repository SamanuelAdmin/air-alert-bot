use tokio::time::{sleep, Duration};
use dotenv::dotenv;
use std::env;
use std::collections::{HashMap, HashSet};
use teloxide::prelude::*;
use tera::Context;

// local modules
mod configs;
use configs::get_configs;
mod parser;
use parser::{Parser, Alert};
mod views;
use views::{
    View, TelegramBotView,
    templates
};




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
    let configs = get_configs();
    let templates_manager = templates::TemplatesManager::new(
        configs.default_templates_dir
    )?;
    
    let mut parser = Parser::new(
        "https://api.alerts.in.ua/v1/alerts/active.json".to_string(),
        env_data.alerts_api_token, &configs.tracked_regions
    );

    let mut tg_bot_view = TelegramBotView::new(
        &env_data.bot_api_token, configs.show_updates
    ).await;
    
    for con_chat in configs.tracked_chats {
        tg_bot_view.connect_chat(con_chat).await;
        println!("Connected chat from the configs: {}", con_chat);
    }

    tg_bot_view.start_bot().await;

    // first start flag, if true - nothing will be show after first parse
    // then will be change to false
    let mut first_parse_flag = configs.mut_first_start;

    loop {
        let changed_alerts: Vec<&Alert> = parser.parse().await?;

        for changed_alert in changed_alerts {
            if first_parse_flag {
                first_parse_flag = false;
                break;
            }

            let mut context = Context::new();
            context.insert("alert", changed_alert);
            tg_bot_view.show(
                // &format!("{}", changed_alert)
                &templates_manager.render_template(
                    &configs.template_name, &context
                )?
            ).await?;
            println!("{}", changed_alert);
        }
        

        // blocking before next request
        sleep(Duration::from_millis(
                (configs.requests_timeout * 1000).into()
            )).await;
    }

    Ok(())
}
