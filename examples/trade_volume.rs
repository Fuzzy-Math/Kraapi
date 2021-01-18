use krakenapi::private::*;
use krakenapi::api::*;
use krakenapi::api::PairList;
use krakenapi::client::KrakenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "KUah/CxtCrxxPwdGLrJ2n4U3iK4mj7kMec/F6Ka9+5YeuHLSxQG6fVUG",
        "/8r4xWSGp1PJE64yz4j1icKZUjaRvEhc2pGmxUbVrkRFmAgEcA0O+CrN1zFR2TKUoBD2Ar3vKM9zyNb0+KH/UA=="
    );
/*
    let client = KrakenClient::new(
        "xcMeCo3qfu3kjujKxOek6UEjtoceOZC0CrYS9bs3M2ozTXe8qGftODux",
        "AapTPpkW+F4kTRDGMc9AoirfdwgPnzFL/iVH8fUGMMPvAftMRhjd0J0hqMIAmbk3RA3AmLdcxUtqc1Qu2weRyA=="
    );
*/
    let trade_volume = KITradeVolume::build()
    .for_pair_list(vec!(KAssetPair(KAsset::EUR, KAsset::CAD), KAssetPair(KAsset::EUR, KAsset::AUD)))
    .with_fee_info()
    .finish_input();
    let res = client.request(&trade_volume).await?;

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    println!("body: {:?}", buf);
    Ok(())
}

