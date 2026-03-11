use reqwest::Client;


pub struct AlertsApiClient {
    url: String,
    token: String,
    client: Client
}

impl AlertsApiClient {
    pub fn new(url: String, token: String) -> Self {
        Self {
            url: url,
            token: token,
            client: Client::new()
        }
    }

    pub async fn get(&self) -> Result<String, Box<dyn std::error::Error>> {
        let request = match self.client.get(
                &(format!("{}?token={}", self.url, self.token))
            ).send().await {
                Ok(response) => response,
                Err(error) => {
                    return Err(Box::new(error));
                }
            };

        let data = match request.text().await {
            Ok(response) => response,
            Err(error) => {
                return Err(Box::new(error));
            }
        };

        return Ok(data);
    }
}
