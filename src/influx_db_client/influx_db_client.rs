use std::env;
use std::string::String;

pub struct InfluxDbClient {
    pub influx_db_address: String,
    pub http_client: reqwest::Client,
}

impl InfluxDbClient {
    pub async fn send_payload(&self, payload: String) -> Result<(), reqwest::Error> {
        let response = self.http_client
            .post(&self.influx_db_address)
            .body(payload.into_bytes())
            .header("Content-Type", "application/octet-stream")
            .send()
            .await;
        println!("{:?}", response);
        Ok(())
    }
}

impl Default for InfluxDbClient {
    fn default() -> Self {
        InfluxDbClient{
            influx_db_address: get_env_var("INFLUX_DB_ADDRESS"),
            http_client: reqwest::Client::new(),
        }
    }
}

fn get_env_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val.to_string(),
        Err(error) => error.to_string(),
    }
}
