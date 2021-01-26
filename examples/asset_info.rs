use krakenapi::public::*;
use krakenapi::api::*;
use krakenapi::client::KrakenClient;
use krakenapi::api::Input;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "<Your-API-Key>",
        "<Your-API-Secret>"
    );

    let asset_info = KIAssetInfo::build()
        //.asset(KAsset::XBT)
        //.asset(KAsset::USD)
        .with_asset_list(vec!(KAsset::XBT, KAsset::USD))
        .finish();
    let res = client.request(&asset_info).await?;

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    //let v: Value = serde_json::from_slice(&buf)?;
    let v: KrakenResult<KOAssetInfo> = serde_json::from_slice(&buf)?;
    println!("body: {:?}", v);
    println!();
    println!();

    let asset_pairs = KIAssetPairs::build()
        .with_asset_pair(KAssetPair(KAsset::EUR, KAsset::CAD))
        .finish();
    let res = client.request(&asset_pairs).await?;

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    //let v: Value = serde_json::from_slice(&buf)?;
    let v: KrakenResult<KOAssetPairInfo> = serde_json::from_slice(&buf)?;
    println!("body: {:?}", v);
    
    Ok(())
}

