use krakenapi::api::private;
use krakenapi::client::KrakenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "l2ln+ZDLJFx6EJb0UO8svBeZzk+7JLhpV1jdCgrxupEeQFsyuBIaF3bv",
        "x4pidRAMqi1s6zg2I3JbeR6C2OAr/BVataXf+hiNa9pbS8vTMndraTLg9O2cZa+hDfQPYtBjBN55NXo8kf/GcQ=="
        );

    let res = private::get_server_time(&client).await?;
    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    println!("body: {:?}", buf);
    Ok(())
}
