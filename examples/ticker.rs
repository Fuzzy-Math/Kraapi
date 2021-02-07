use krakenapi::api::*;
use krakenapi::public::*;
use krakenapi::client::KrakenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "",
        ""
    );

    let ticker = KITicker::build(KAssetPair(KAsset::XBT, KAsset::USD))
        .finish();

    // Both of these do the same thing
    let res = client.request::<KOTicker>(&ticker).await?;
    //let res: KOTicker = client.request(&ticker).await?;

    println!("{:?}", res);

    Ok(())
}
