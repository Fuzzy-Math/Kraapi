use krakenapi::private::*;
use krakenapi::private::*;
use krakenapi::api::*;
use krakenapi::client::KrakenClient;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "<Your-API-Key>",
        "<Your-API-Secret>"
    );

    //let res = private::get_trade_balance(&client).await?;
    let trade_balance = KITradeBalance::build()
        .with_asset(KAsset::XBT)
        .finish();
    let res = client.request(&trade_balance).await?;

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    //let v: Value = serde_json::from_slice(&buf)?;
    let v: KrakenResult<KOTradeBalance> = serde_json::from_slice(&buf)?;
    println!("body: {:?}", v);
    Ok(())
}

