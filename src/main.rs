use log::{debug, info};
use tokio::time::{self, Duration};
mod hue_client;
mod influx_db_client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    let influxdb_client = influx_db_client::InfluxDbClient {
        ..Default::default()
    };
    let hue_client = hue_client::HueClient {
        ..Default::default()
    };
    info!("Initialising HueReader");

    let mut interval = time::interval(Duration::from_secs(300));
    loop {
        let now = interval.tick().await;
        debug!("Ticker: (Time now = {:?})", now);

        let hue_bridges = hue_client.discover_bridges().await;
        debug!("hue_bridges is: {:?}", hue_bridges);
        let hue_resources = hue_client.scan_resources(hue_bridges?).await;
        debug!("hue_resources is: {:?}", hue_resources);
        for item in hue_resources? {
            let _response = influxdb_client.send_payload(item).await;
        }
    }
}
