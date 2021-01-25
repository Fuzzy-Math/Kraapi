use krakenapi::public::*;
use krakenapi::api::*;
use krakenapi::client::KrakenClient;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "KUah/CxtCrxxPwdGLrJ2n4U3iK4mj7kMec/F6Ka9+5YeuHLSxQG6fVUG",
        "/8r4xWSGp1PJE64yz4j1icKZUjaRvEhc2pGmxUbVrkRFmAgEcA0O+CrN1zFR2TKUoBD2Ar3vKM9zyNb0+KH/UA=="
    );

    //let res = private::get_account_balance(&client).await?;
    let time = KIServerTime::build();
    let status = KISystemStatus::build();
    let timeres = client.request(&time).await?;

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(timeres).await?;
    //let v: Value = serde_json::from_slice(&buf)?;
    let v: KrakenResult<ServerTime> = serde_json::from_slice(&buf)?;
    println!("body: {:?}", v);

    let statusres = client.request(&status).await?;

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(statusres).await?;
    let v: Value = serde_json::from_slice(&buf)?;
    //let v: KrakenResult<SystemStatus> = serde_json::from_slice(&buf)?;
    println!("body: {:?}", v);

    Ok(())
}
