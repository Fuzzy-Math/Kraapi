use krakenapi::api::*;
use krakenapi::client::KrakenClient;
use krakenapi::private::*;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new("<Your-API-Key>", "<Your-API-Secret>");

    let trade_volume = KITradeVolume::build().with_fee_info(true).finish();
    let res = client.request(&trade_volume).await?;

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    //let v: Value = serde_json::from_slice(&buf)?;
    let v: KrakenResult<KOTradeVolume> = serde_json::from_slice(&buf)?;
    println!("body: {:?}", v);
    Ok(())
}
