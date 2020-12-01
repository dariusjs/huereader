use serde_json::Value;
use std::env;
use std::string::String;
mod model;

fn get_env_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val.to_string(),
        Err(error) => error.to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let hue_api_key = get_env_var("HUE_API_KEY");
    let influx_db_address = get_env_var("INFLUX_DB_ADDRESS");
    let hue_discovery_url = "https://discovery.meethue.com/";
    let response = reqwest::get(hue_discovery_url).await?;
    assert!(response.status().is_success());

    let bridge_body = response.text().await?;
    let data: Value = serde_json::from_str(&bridge_body).unwrap();
    let hue_bridges: Vec<model::HueBridge> = serde_json::from_value(data).unwrap();
    let hue_http_client = reqwest::Client::new();
    let influx_db_client = reqwest::Client::new();

    for bridge in hue_bridges {
        let hue_sensors_url = format!("http://{}/api/{}/", bridge.internalipaddress, hue_api_key);
        let response = hue_http_client.get(&hue_sensors_url).send().await?;
        let bridge_scan_body = response.text().await?;

        let hue_resources: model::HueResources = serde_json::from_str(&bridge_scan_body).unwrap();
        // let mut sensor_list = vec![];
        for (_, item) in hue_resources.sensors {
            let sensor = item.payload();
            // println!("sensor is: {:#?}", sensor);
            if sensor != "" {
                // sensor_list.push(sensor);

                let temperature_response = influx_db_client
                    .post(&influx_db_address)
                    .body(sensor.into_bytes())
                    .header("Content-Type", "application/octet-stream")
                    .send()
                    .await?;
                assert!(temperature_response.status().is_success());
                println!("response is: {:?}", temperature_response);
            }
        }
    }
    Ok(())
}
