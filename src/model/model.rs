use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::String;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
            crate::model::HueSensors::ZLLTemperature(hue_temp_sensor) => {
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
            crate::model::HueSensors::ZLLLightLevel(hue_light_sensor) => {
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
            _ => (),
        }
        return payload.to_string();
    }
}
