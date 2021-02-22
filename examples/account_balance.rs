use krakenapi::api::*;
use krakenapi::private::*;
use krakenapi::client::KrakenClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::new(
        "<Your-API-Key>",
        "<Your-API-Secret>"
    );

    let account_balance = KIAccountBalance::build();

    let res = client.request::<KOAccountBalance>(&account_balance).await?;

    println!("{:?}", res);

    Ok(())
}
