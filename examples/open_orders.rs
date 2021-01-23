use krakenapi::api::*;
use krakenapi::private::*;
use krakenapi::client::KrakenClient;
use serde_json::Value;

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

    //let res = private::get_trade_balance(&client).await?;
    let open_orders = KIOpenOrders::build()
        .with_trade_info(true)
        .finish();
    let res1 = client.request(&open_orders).await?;

    // Concatenate the body stream into a single buffer...
    let buf1 = hyper::body::to_bytes(res1).await?;
    //let v: Value = serde_json::from_slice(&buf)?;
    let v1: KrakenResult<OpenOrders> = serde_json::from_slice(&buf1)?;
    println!("{:?}", v1);

    let closed_orders = KIClosedOrders::build()
        .from_txid(String::from("OSZ3EZ-UIDEH-LRTIXW"))
        .to_txid(String::from("OZIDYB-CLNS5-PGGLZG"))
        .finish();
    let res2 = client.request(&closed_orders).await?;
    let buf2 = hyper::body::to_bytes(res2).await?;
    //let v2: Value = serde_json::from_slice(&buf2)?;
    let v2: KrakenResult<ClosedOrders> = serde_json::from_slice(&buf2)?;
    println!("{:?}", v2);

    let orders = vec!(String::from("OSZ3EZ-UIDEH-LRTIXW"), String::from("OZIDYB-CLNS5-PGGLZG"));
    let queried_orders = KIOrderInfo::build_with_list(orders)
        .finish();
    let res3 = client.request(&queried_orders).await?;
    let buf3 = hyper::body::to_bytes(res3).await?;
    //let v3: Value = serde_json::from_slice(&buf3)?;
    let v3: KrakenResult<QueriedOrders> = serde_json::from_slice(&buf3)?;
    println!("{:?}", v3.result.unwrap().get("OZIDYB-CLNS5-PGGLZG").unwrap().descr.pair);
    Ok(())
}
