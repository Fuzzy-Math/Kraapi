use krakenapi::api::*;
use krakenapi::public::*;
use krakenapi::client::KrakenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "",
        ""
    );

    let order_book = KIOrderBook::build(KAssetPair(KAsset::XBT, KAsset::USD)).finish();

    // Both of these do the same thing
    let res = client.request::<KOOrderBook>(&order_book).await?;
    //let res: KOTicker = client.request(&ticker).await?;

    println!("{:?}\n\n", res);

    let recent_trades = KIRecentTrades::build(KAssetPair(KAsset::XBT, KAsset::USD)).finish();

    let res = client.request::<KORecentTrades>(&recent_trades).await?;
    println!("{:?}\n\n", res);

    let spread_data = KISpreadData::build(KAssetPair(KAsset::XBT, KAsset::USD)).finish();

    let res = client.request::<KOSpreadData>(&spread_data).await?;
    println!("{:?}", res);

    Ok(())
}
