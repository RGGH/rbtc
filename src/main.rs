use reqwest;
use std::error::Error;
use std::time::Duration;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let btc = client
        .get("https://api.coinstats.app/public/v1/coins/bitcoin")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

    // convert response to json
    let json: Value = serde_json::from_str(&btc).unwrap_or_else(|e| {
            panic!("Failed to parse json; error is {}", e);
        });
    
    let price = json["coin"]["price"].to_string();
    let price = price.split_once('.').unwrap();

    println!("Bitcoin Price {:?}", price.0);
  

    Ok(())
}