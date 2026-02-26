/* 

Module, responsible for making requests
to the alerts.in.ua and giving data in a
simple and easy-to-use way (Alert structure)

*/


use std::collections::HashMap;
use std::fmt;
use reqwest::Client;
use serde_json::{Value};

// local imports
mod alert;
pub use alert::Alert;
mod http_client;
use http_client::AlertsApiClient;



pub struct Parser {
    alerts: HashMap<u32, Alert>,
    api_client: AlertsApiClient
}


impl Parser {
    pub fn new(url: String, alerts_api_token: String, regions: &[u32]) -> Self{
        let mut alerts: HashMap<u32, Alert> = HashMap::with_capacity(regions.len());
    
        for region in regions {
            alerts.insert(*region, Alert::new());
        }   

        Self { 
            alerts: alerts,
            api_client: AlertsApiClient::new( url, alerts_api_token )
        }
    }


    pub async fn parse(&mut self)
        -> Result<Vec<&Alert>, Box<dyn std::error::Error>> {
        // returns vector with changed Alerts 
        let mut result: Vec<&Alert> = Vec::new();
            
        let parsing_data: String = self.api_client.get().await?;
        let changed = self.parse_from_string(parsing_data).await?;

        for changed_id in changed {
            result.push(&self.alerts[&changed_id]);
        }

        return Ok(result);
    }


    async fn parse_from_string(&mut self, data: String)
        -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let json_data: Value = serde_json::from_str(&data)?;

        // data part
        let alerts_info = json_data["alerts"].as_array()
            .ok_or("Cannot find data part in JSON.")?;
        let mut changed_list: Vec<u32> = Vec::new();
        let mut active_list: Vec<u32> = Vec::new();


        for alert_info in alerts_info {
            let alert_id = alert_info["id"].as_u64()
                .ok_or("Cannot parse id of alert.")?
                .try_into()?;
            active_list.push(alert_id);
    
            if let Some(alert) = self.alerts.get_mut(&alert_id) {
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
        for alert in self.alerts.values_mut() {
            if !active_list.contains(&alert.id) && alert.state {
                alert.deactivate();
                changed_list.push(alert.id);
            }
        }
    
        return Ok(changed_list);
    } 
}

