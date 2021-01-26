use krakenapi::api::private;
use krakenapi::client::KrakenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "<Your-API-Key>",
        "<Your-API-Secret>"
    );

    let res = private::get_server_time(&client).await?;
    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;
    println!("body: {:?}", buf);
    Ok(())
}
