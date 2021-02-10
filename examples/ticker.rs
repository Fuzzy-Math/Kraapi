use krakenapi::api::*;
use krakenapi::public::*;
use krakenapi::client::KrakenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "",
        ""
    );

    let ticker = KITicker::build_with_list(vec!(
            KAssetPair(KAsset::XBT, KAsset::USD), 
            KAssetPair(KAsset::XBT, KAsset::CAD)
            ))
        .finish();

    // Both of these do the same thing
    let res = client.request::<KOTicker>(&ticker).await?;
    //let res: KOTicker = client.request(&ticker).await?;

    println!("{:?}", res);

    let ohlc = KIOHLC::build(KAssetPair(KAsset::XBT, KAsset::USD))
        .with_interval(OHLCInterval::TenEighty)
        .finish();

    let res = client.request::<KOOHLC>(&ohlc).await?;
    println!("{:?}", res);

    Ok(())
}
