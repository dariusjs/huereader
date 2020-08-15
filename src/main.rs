// #[tokio::main]
use serde_json::{Result, Value};

// async fn main() -> Result<(), reqwest::Error> {
//     let response = reqwest::get("https://discovery.meethue.com/").await?;

//     println!("Status: {}", response.status());

//     let body = response.text().await?;

//     println!("Body:\n\n{}", body);


//     // Parse the string of data into serde_json::Value.
//     let v: Value = serde_json::from_str(body[0])?;

//     // Access parts of the data by indexing with square brackets.
//     println!("Please call {}", v["internalipaddress"]);

//     Ok(())
// }


fn main() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
    {"id":"someid","internalipaddress":"192.168.100.100"}"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} ", v["internalipaddress"]);

    Ok(())
}

// fn main() {
//     println!("Hello, world!");
//     let hue_address = get_hue_address();
//     println!("Body:\n\n{}", hue_address);
// }
