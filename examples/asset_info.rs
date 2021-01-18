use krakenapi::public::*;
use krakenapi::api::*;
use krakenapi::client::KrakenClient;
use krakenapi::api::Input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let client = KrakenClient::new(
    //    "l2ln+ZDLJFx6EJb0UO8svBeZzk+7JLhpV1jdCgrxupEeQFsyuBIaF3bv",
    //    "x4pidRAMqi1s6zg2I3JbeR6C2OAr/BVataXf+hiNa9pbS8vTMndraTLg9O2cZa+hDfQPYtBjBN55NXo8kf/GcQ=="
    //    );

    let client = KrakenClient::new(
        "xcMeCo3qfu3kjujKxOek6UEjtoceOZC0CrYS9bs3M2ozTXe8qGftODux",
        "AapTPpkW+F4kTRDGMc9AoirfdwgPnzFL/iVH8fUGMMPvAftMRhjd0J0hqMIAmbk3RA3AmLdcxUtqc1Qu2weRyA=="
    );

    let asset_info = KIAssetInfo::build()
        //.asset(KAsset::XBT)
        //.asset(KAsset::USD)
        .asset_list(vec!(KAsset::XBT, KAsset::USD))
        .finish_input();
    let res = client.request(&asset_info).await?;

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    println!("body: {:?}", buf);
    println!();
    println!();

    let asset_pairs = KIAssetPairs::build()
        .pair(KAssetPair(KAsset::EUR, KAsset::CAD))
        .info(APairInfo::MARGIN)
        .finish_input();
    let res = client.request(&asset_pairs).await?;

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    println!("body: {:?}", buf);
    
    Ok(())
}

