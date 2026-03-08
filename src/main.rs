use tokio::time::{sleep, Duration};
use dotenv::dotenv;
use std::env;
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


async fn tg_bot_show(
    tg_bot_view: &mut TelegramBotView, templates_manager: &templates::TemplatesManager, 
    template_name: &str, alerts: &Vec<&Alert>
) -> Result<(), Box<dyn std::error::Error>> {
    // show function for telegram bot
    let mut context = Context::new();
    context.insert("alerts", alerts);
    
    let render = templates_manager.render_template(
            template_name, &context
        )?;

    println!("{:?}", alerts);
    println!("Render: {}", render);

    // views part 
    tg_bot_view.show(&render).await?;

    Ok(())
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
    println!("Systems had been inited and started.");


    async fn parsing_delay(delay: &u16) {
        sleep(Duration::from_millis(
                (delay * 1000).into()
            )).await;
    }

    // first start flag, if true - nothing will be show after first parse
    // then will be change to false
    let mut first_parse_flag = configs.mut_first_start;

    loop {
        let changed_alerts: Vec<&Alert> = parser.parse().await?;
        if changed_alerts.len() <= 0 {
            parsing_delay(&configs.requests_timeout).await;
            continue; // skip empty lists
        }

        if first_parse_flag {
            first_parse_flag = false;
            
            // blocking before next request
            parsing_delay(&configs.requests_timeout).await;
            continue;
        }
        
        // context makers 
        let mut alerts_active: Vec<&Alert> = Vec::new();
        let mut alerts_deactive: Vec<&Alert> = Vec::new();

        for changed_alert in &changed_alerts {
            // you can also add some views here,
            // if you need to process every message
            
            if changed_alert.state {
                alerts_active.push(changed_alert);
            } else {
                alerts_deactive.push(changed_alert);
            }

            println!("{}", changed_alert);
        }

        if alerts_active.len() > 0 {
            tg_bot_show(
                &mut tg_bot_view, &templates_manager, 
                &configs.template_name, &alerts_active
            ).await?;
        }
        if alerts_deactive.len() > 0 {
            tg_bot_show(
                &mut tg_bot_view, &templates_manager, 
                &configs.template_name, &alerts_deactive
            ).await?;
        }
        
        
        // blocking before next request
        parsing_delay(&configs.requests_timeout).await;
    }
}
