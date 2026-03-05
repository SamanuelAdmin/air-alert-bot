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
        &env_data.bot_api_token
    ).await;
    tg_bot_view.connect_chat(1178323450).await;
    tg_bot_view.start_bot().await;


    loop {
        let changed_alerts: Vec<&Alert> = parser.parse().await?;

        for changed_alert in changed_alerts {
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
