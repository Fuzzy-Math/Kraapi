use krakenapi::public::server_time::{ KIServerTime, KOServerTime };
use krakenapi::public::system_status::{ KISystemStatus, KOSystemStatus };
use krakenapi::client::KrakenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new("", "");

    println!("body: {:?}", client.request::<KOServerTime>(&KIServerTime::build()).await?);
    println!("body: {:?}", client.request::<KOSystemStatus>(&KISystemStatus::build()).await?);

    Ok(())
}
