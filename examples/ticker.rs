use krakenapi::api::*;
use krakenapi::client::KrakenClient;
use krakenapi::public::ohlc::{KIOHLC, KOOHLC};
use krakenapi::public::ticker::{KITicker, KOTicker};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new("", "");

    let ticker = KITicker::build_with_list(vec![
        KAssetPair(KAsset::XBT, KAsset::USD),
        KAssetPair(KAsset::XBT, KAsset::CAD),
    ])
    .finish();

    // Both of these do the same thing
    //let res = client.request::<KOTicker>(&ticker).await?;
    //let res: KOTicker = client.request(&ticker).await?;

    //println!("{:?}", res);

    let ohlc = KIOHLC::build(KAssetPair(KAsset::XBT, KAsset::USD))
        .with_interval(OHLCInterval::Five)
        .since(String::from("1614750200"))
        .finish();

    //let res = client.request::<KOOHLC>(&ohlc).await?;
    //println!("{:#?}", res);

    match client.request::<KOOHLC>(&ohlc).await {
        Ok(res) => {
            if let Some(data) = res.pair.get("XXBTZUSD") {
                for (index, data) in data.iter().enumerate() {
                    println!(
                        "Index: {}  OHLC: ({}, {}, {}, {})",
                        index, data.open, data.high, data.low, data.close
                    );
                }
            } else {
                println!("Asset pair missing from hash map");
            }
        }

        Err(errors) => {
            for error in errors {
                println!("{}", error);
            }
        }
    }

    Ok(())
}
