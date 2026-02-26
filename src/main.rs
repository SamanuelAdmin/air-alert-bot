use tokio::time::{sleep, Duration};
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use std::fmt;
use reqwest::Client;
use serde_json::{Value};
use teloxide::prelude::*;

// name was taken from official API site alerts.in.ua
struct Alert {
    id: u32, // if id == 0 -> Alert is empty
    location_oblast_uid: u32,
    state: bool,
    location_title: String,
}


impl Alert {
    fn new() -> Self {
        Self {
            id: 0,
            location_oblast_uid: 0,
            state: false,
            location_title: String::new()
        }
    }


    fn init(
        &mut self, id: u32, location_oblast_uid: u32, location_title: String, 
    ) {
        if self.id == 0 {()}

        self.id = id;
        self.location_oblast_uid = location_oblast_uid;
        self.location_title = location_title;
    }

    #[inline]
    fn activate(&mut self) {
        self.state = true;
    }

    #[inline]
    fn deactivate(&mut self) {
        self.state = false;
    }
}



impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.id == 0 {
            return write!(f, "Empty Alert.");
        }
        return write!(f, "[{}] {} - {}", self.id, self.location_title, self.state);
    }
}



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



fn init_alerts(alerts: &mut HashMap<u32, Alert>, tracked_regions: &[u32]) {
    for region in tracked_regions {
        alerts.insert(*region, Alert::new());
    }
}


// just a http module! do not parse json here!!!
async fn get_alerts_data(client: &Client, url: &str, token: &str) -> Result<String, String> {

    let request = match client.get(
            &(format!("{}?token={}", url, token))
        ).send().await {
            Ok(response) => response,
            Err(error) => return Err(
                format!("Got error when making request: {error}")
            )
        };

    let data = match request.text().await {
        Ok(response) => response,
        Err(error) => return Err(
            format!("Error with getting text: {error}")
        )
    };

    println!("{}", data);
    return Ok(data);

}

fn parse_alerts(
        alerts: &mut HashMap<u32, Alert>, parsed_data: Value
    ) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    // returns list with changed alerts
    let alerts_info = parsed_data["alerts"].as_array()
        .ok_or("Cannot find data part in JSON.")?;
    let mut changed_list: Vec<u32> = Vec::new();
    let mut active_list: Vec<u32> = Vec::new();

    for alert_info in alerts_info {
        let alert_id = alert_info["id"].as_u64()
            .ok_or("Cannot parse id of alert.")?
            .try_into()?;
        active_list.push(alert_id);

        if let Some(alert) = alerts.get_mut(&alert_id) {
            // fill info about alert`s location
            if alert.id == 0 { 
                let title = alert_info["location_title"].as_str()
                    .ok_or("Cannot get location_title from JSON.")?;
                let obl_uid = alert_info["location_oblast_uid"].as_u64()
                    .ok_or("Cannot get oblast_uid from JSON.")?;

                alert.init(
                    alert_id,
                    obl_uid.try_into()?,
                    title.to_owned()
                );
            }
            
            if !alert.state {
                changed_list.push(alert_id);
                alert.activate();
            }
        }
    }

    // deactivate unactive alerts
    for alert in alerts.values_mut() {
        if !active_list.contains(&alert.id) && alert.state {
            alert.deactivate();
            changed_list.push(alert.id);
        }
    }

    return Ok(changed_list);
}


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

    let mut alerts: HashMap<u32, Alert> = HashMap::with_capacity(TRACKED_REGIONS.len());
    init_alerts(&mut alerts, &TRACKED_REGIONS);
    
    let client = Client::new();


    let bot = Bot::new(env_data.bot_api_token).auto_send();
    let mut chat_list: Vec<ChatId> = Vec::new();

    // for test only!!!
    chat_list.push(ChatId(1178323450));

    loop {
        let value = get_alerts_data(
                    &client,
                    "https://api.alerts.in.ua/v1/alerts/active.json",
                    &env_data.alerts_api_token
                ).await?;
        
        let changed_alerts = parse_alerts(
            &mut alerts, serde_json::from_str(&value)?
        )?;

        for alert_id in &changed_alerts {
            for chat_id in &chat_list {
                bot.send_message(
                    *chat_id, format!("{}", &alerts[alert_id])
                ).await?;
            }
            println!("{}", &alerts[alert_id]);
        }
        

        // blocking for 5 seconds
        sleep(Duration::from_millis(10000)).await;
    }

    Ok(())
}
