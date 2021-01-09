use krakenapi::api::private;
use krakenapi::client::KrakenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let client = KrakenClient::new(
      //  "l2ln+ZDLJFx6EJb0UO8svBeZzk+7JLhpV1jdCgrxupEeQFsyuBIaF3bv",
        //"x4pidRAMqi1s6zg2I3JbeR6C2OAr/BVataXf+hiNa9pbS8vTMndraTLg9O2cZa+hDfQPYtBjBN55NXo8kf/GcQ=="
       // );
    let client = KrakenClient::new(
        "HjiS5SPLVzwzKdKabggcME52AEAf6NMhKFmphH5bG6I7GFp4obsxRrRF",
        "C8b+t5353uBz103dBnAtEQds6jkbc3NWSSVS01iwDxqRiV8w8UuuLQOF5eeKeqQ3i6WROp+RfH51/cUVZeywhQ=="
    );

    let res = private::get_account_balance(&client).await?;
    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    println!("body: {:?}", buf);
    Ok(())
}
