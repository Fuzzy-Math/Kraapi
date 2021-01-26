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

    let account_balance = KIAccountBalance::build();
    let res = client.request(&account_balance).await?;
    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    //let v: Value = serde_json::from_slice(&buf)?;
    let v: KrakenResult<KOAccountBalance> = serde_json::from_slice(&buf)?;
    //println!("body: {:?}", buf);
    println!("{:?}", v);
    Ok(())
}
