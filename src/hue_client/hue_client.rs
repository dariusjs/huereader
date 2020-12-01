use serde_json::Value;
use std::env;
use serde::{Deserialize, Serialize};
use std::string::String;
use std::collections::HashMap;

pub struct HueClient {
    pub hue_discovery_url: String,
    pub hue_api_key: String,
    pub http_client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HueBridge {
    id: String,
    pub internalipaddress: String,
}

#[derive(Debug, Deserialize)]
pub struct HueResources {
    // //commented out structures we are not implementing yet
    // config: String,
    // scenes: String,
    // schedules: String,
    pub sensors: HashMap<String, HueSensors>,
    // resourcelinks: String,
    // lights: String,
    // rules: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum HueSensors {
    ZLLTemperature(HueTempSensor),
    ZLLLightLevel(HueLightSensor),
    Daylight(HueGenericSensor),
    ZLLPresence(HueGenericSensor),
    CLIPGenericFlag(HueGenericSensor),
    ZLLSwitch(HueGenericSensor),
    CLIPGenericStatus(HueGenericSensor),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct HueTempSensor {
    pub config: HueSensorConfig,
    pub name: String,
    pub state: HueTempSensorState,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HueTempSensorState {
    pub temperature: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct HueLightSensor {
    pub config: HueSensorConfig,
    pub name: String,
    pub state: HueLightSensorState,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HueLightSensorState {
    pub lightlevel: f64,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct HueGenericSensor {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HueSensorConfig {
    pub battery: f64,
}

impl HueSensors {
    pub fn payload(self) -> String {
        let payload = "";
        match self {
            crate::hue_client::HueSensors::ZLLTemperature(hue_temp_sensor) => {
                let device_temperature: f64;
                let device_battery: f64;
                let device_name = hue_temp_sensor
                    .name
                    .to_string()
                    .replace(" ", "_")
                    .replace("\"", "");
                let config = hue_temp_sensor.config;
                match config {
                    battery => {
                        device_battery = battery.battery;
                    }
                }
                let state = hue_temp_sensor.state;
                match state {
                    temperature => {
                        device_temperature = temperature.temperature / 100.0;
                    }
                }
                let payload = format!(
                    "hue,name={} temperature={:#?},battery={:#?}",
                    device_name, device_temperature, device_battery
                );
                return payload;
            }
            crate::hue_client::HueSensors::ZLLLightLevel(hue_light_sensor) => {
                let mut lux: f64;
                let device_battery: f64;
                let device_name = hue_light_sensor
                    .name
                    .to_string()
                    .replace(" ", "_")
                    .replace("\"", "");
                let config = hue_light_sensor.config;
                match config {
                    battery => {
                        device_battery = battery.battery;
                    }
                }
                let state = hue_light_sensor.state;
                match state {
                    lightlevel => {
                        lux = ((lightlevel.lightlevel - 1.0) / 10000.0).log10();
                        if lux.is_nan() {
                            lux = 0.0;
                        }
                    }
                }
                let payload = format!(
                    "hue,name={} lux={:#?},battery={:#?}",
                    device_name, lux, device_battery
                );
                return payload;
            }
            _ => ()
        }
        return payload.to_string();
    }
}

impl HueClient {
    pub async fn discover_bridges(&self) -> Result<Vec<HueBridge>, reqwest::Error> {
        let response = reqwest::get(&self.hue_discovery_url).await?;
        assert!(response.status().is_success());    
        let bridge_body = response.text().await?;
        let data: Value = serde_json::from_str(&bridge_body).unwrap();
        let hue_bridges: Vec<HueBridge> = serde_json::from_value(data).unwrap();
        Ok(hue_bridges)
    }
    pub async fn scan_resources(&self, hue_bridges: Vec<HueBridge>) -> Result<Vec<std::string::String>, reqwest::Error> {
        let mut sensor_list = vec![];
        for bridge in hue_bridges {
            let hue_sensors_url = format!("http://{}/api/{}/", bridge.internalipaddress, self.hue_api_key);
            let response = self.http_client.get(&hue_sensors_url).send().await?;
            let bridge_scan_body = response.text().await?;
    
            let hue_resources: HueResources = serde_json::from_str(&bridge_scan_body).unwrap();
            for (_, item) in hue_resources.sensors {
                let sensor = item.payload();
                if sensor != "" {
                    sensor_list.push(sensor);
                }
            }
        }
        Ok(sensor_list)        
    }
}

fn get_env_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val.to_string(),
        Err(error) => error.to_string(),
    }
}

impl Default for HueClient {
    fn default() -> Self {
        HueClient{
            hue_discovery_url: "https://discovery.meethue.com/".to_string(),
            hue_api_key: get_env_var("HUE_API_KEY"),
            http_client: reqwest::Client::new(),
        }
    }
}
