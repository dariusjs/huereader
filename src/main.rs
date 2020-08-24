use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::env;
use std::string::String;

#[derive(Debug, Serialize, Deserialize)]
struct HueBridge {
    id: String,
    internalipaddress: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HueSensorStatus {
    lastupdated: String,
    temperature: i32,
}

fn get_env_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val.to_string(),
        Err(error) => error.to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Get the Hue API Key
    let hue_api_key = get_env_var("HUE_API_KEY");
    let influx_db_address = get_env_var("INFLUX_DB_ADDRESS");
    println!("the api key is: {:?}", hue_api_key);
    let hue_discovery_url = "https://discovery.meethue.com/";
    
    let response = reqwest::get(hue_discovery_url).await?;
    println!("Status: {}", response.status());

    let body = response.text().await?;
    let data: Value = serde_json::from_str(&body).unwrap();
    let hue_bridges: Vec<HueBridge> = serde_json::from_value(data).unwrap();
    
    let hue_http_client = reqwest::Client::new();
    let influx_db_client = reqwest::Client::new();

    for bridge in hue_bridges {
        let hue_sensors_url = format!("http://{}/api/{}/sensors/",
                               bridge.internalipaddress,
                               hue_api_key);
        let response = hue_http_client.get(&hue_sensors_url).send().await?;
        let body = response.text().await?;

        let hue_sensors: Value = serde_json::from_str(&body).unwrap();
        let hue_resource = "type";
       
        for (_key, value) in hue_sensors.as_object().unwrap().iter() {
            let result = search_json(&value, hue_resource);
            if result == "\"ZLLTemperature\""{
                println!("value is: {}", value);
                let device_name = &value.get("name").unwrap().to_string().replace(" ", "_").replace("\"", "");
                let device_battery_level = &value.get("config").unwrap().get("battery").unwrap().clone();
                let device_temperature = &value.get("state").unwrap().get("temperature").unwrap().clone();
                let real_temperature = device_temperature.as_f64().unwrap() / 100.0;
                println!("temperature is: {}", real_temperature);

                let temperature_data = format!("temperature,name={} value={}", device_name, real_temperature);
                let battery_data = format!("battery,name={} value={}", device_name, device_battery_level);
                println!("temperature_data is: {}", temperature_data);
                println!("battery_data is: {}", battery_data);

                let battery_response = influx_db_client.post(&influx_db_address)
                .body(battery_data)
                .header("Content-Type", "application/octet-stream")
                .send()
                .await?;
                println!("response is: {:?}", battery_response);
                let temperature_response = influx_db_client.post(&influx_db_address)
                .body(temperature_data)
                .header("Content-Type", "application/octet-stream")
                .send()
                .await?;
                println!("response is: {:?}", temperature_response);
            }
        }
    }
    Ok(())
}

fn search_json<'a>(hue_sensors: &serde_json::value::Value, searched_key: &'a str) -> String {
    let mut item = "".to_string();
    if hue_sensors.as_object().unwrap().contains_key(searched_key) {
        return hue_sensors[searched_key].to_string();
    }
    for (_key, value) in hue_sensors.as_object().unwrap().iter() {
        if value.is_object() {
            item = search_json(&value, searched_key).to_string();
        }
    }
    return item;
}
