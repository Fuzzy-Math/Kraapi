# Kraapi

Asynchronous HTTP client for the Kraken cryptocurrency exchange

# Features

- Asynchronous
- Type-driven and self-validating API
- Ergonomic and easy to use
- Promotes re-use of structures and avoids unecessary allocations or redundant HTTP clients

# General Notes - TLDR

- Every input type is prefixed with KI. Every output type is prefixed with KO
- Every input type is a builder type for KrakenInput. All required paramters(per Kraken) are
  parameters for that type's constructor. Optional parameters are exposed using methods.
- Only a KrakenInput instance can be passed into the KrakenClient. You must fufill any
  contracts exposed by the type and convert it to a KrakenInput by calling finish() or
  finish_clone() which exists for every input type
- You must await the call to request
- Deserializing the data returned from Kraken into output structs is done for you. Currently if
  you pass in the wrong ouput type, the parsing will fail
- Builder methods require ownership so if you must perform some application logic while
  building a KrakenInput you must reassign the variable like so:
```
use krakenapi::api::AssetPairInfo;
use krakenapi::public::KIAssetPairs;
let some_application_logic = true;
// mut to allow reassignment based on application logic
let mut input = KIAssetPairs::build();

if some_application_logic {
    input = input.info(AssetPairInfo::Leverage);
} else {
    input = input.info(AssetPairInfo::Margin);
}

// Now of type KrakenInput so we have to rebind the variable
let input = input.finish();
```
# Examples 
See <https://www.kraken.com/features/api#example-api-code-php-lib> for more info on these
examples

## Public Endpoint - Ticker
```
use krakenapi::client::KrakenClient;
use krakenapii::public::{KITicker, KOTicker};
use krakenapi::api::{KAsset, KAssetPair};

async fn main() -> hyper::Result<()> {
	 let client = KrakenClient::new("", "");

	 let ticker_input = KITicker::build(KAssetPair(KAsset::XBT, KAsset::USD)).finish();

	 let ticker_output = client.request::<KOTicker>(&ticker_input).await?;

	 println!("{:?}", ticker_output);
	 Ok(())
}
```
## Private Endpoint - Add Order
```
use krakenapi::client::KrakenClient;
use krakenapii::private::{
    KIAddOrder, KOAddOrder};
use krakenapi::api::{
    KAsset, KAssetPair,
    TradeType, OrderType};

async fn main() -> hyper::Result<()> {
	 let client = KrakenClient::new(
	     "<Your-API-Key>", 
	     "<Your-API-Secret>"
	     );

	 let add_order_input = KIAddOrder::build(
	     KAssetPair(KAsset::XBT, KAsset::USD),
	     TradeType::Buy,
	     OrderType::Limit("101.9901"),
	     2.12345678)
	     .with_leverage((2, 1))
	     .with_closing_order(OrderType::StopLossLimit("#5%", "#10"))
	     .validate()
	     .finish();

	 let add_order_output = client.request::<KOAddOrder>(&add_order_input).await?;

	 println!("{:?}", add_order_output);
	 Ok(())
}
```
# Installation
This crate is not currently on crates.io but will be soon. Until then, add the following line to the dependencies section of your project's Cargo.toml
```
krakenapi = { git = "https://github.com/Fuzzy-Math/KrakenAPI-Rust" }
```
That dependency is pinned to the commit it was downloaded from and will have to be update manually if desired
```
cargo update
```
