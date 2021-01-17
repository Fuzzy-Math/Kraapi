use krakenapi::public::*;
use krakenapi::client::KrakenClient;

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
    println!("Response: {}", timeres.status());
    println!("Headers: {:#?}\n", timeres.headers());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(timeres).await?;
    println!("body: {:?}", buf);

    let statusres = client.request(&status).await?;
    println!("Response: {}", statusres.status());
    println!("Headers: {:#?}\n", statusres.headers());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(statusres).await?;
    println!("body: {:?}", buf);

    Ok(())
}
