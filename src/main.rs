use tokio::time::Duration;
mod hue_client;
mod influx_db_client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let influxdb_client = influx_db_client::InfluxDbClient {
        ..Default::default()
    };
    let hue_client = hue_client::HueClient {
        ..Default::default()
    };

    let mut interval_day = tokio::time::interval(Duration::from_secs(3));
    loop {
        let now = interval_day.tick().await;
        println!("Initialising HueReader. (Time now = {:?})", now);

        let hue_bridges = hue_client.discover_bridges().await;
        println!("hue_bridges is: {:?}", hue_bridges);
        let hue_resources = hue_client.scan_resources(hue_bridges.unwrap()).await;
        println!("hue_resources is: {:?}", hue_resources);
        for item in hue_resources.unwrap() {
            let _response = influxdb_client.send_payload(item).await;
        }
    }
}
