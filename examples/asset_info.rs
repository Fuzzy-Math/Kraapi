use krakenapi::api::Input;
use krakenapi::api::*;
use krakenapi::client::KrakenClient;
use krakenapi::public::*;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new("<Your-API-Key>", "<Your-API-Secret>");

    let asset_info = KIAssetInfo::build()
        .with_asset_list(vec![KAsset::XBT, KAsset::USD])
        .finish();
    let res = client.request::<KOAssetInfo>(&asset_info).await?;

    println!("body: {:?}", res);
    println!();
    println!();

    let asset_pairs = KIAssetPairs::build()
        .with_asset_pair(KAssetPair(KAsset::EUR, KAsset::CAD))
        .finish();
    let res = client.request::<KOAssetPairInfo>(&asset_pairs).await?;

    println!("body: {:?}", res);

    Ok(())
}
