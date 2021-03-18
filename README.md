# Kraapi
[![crates.io](http://meritbadge.herokuapp.com/kraapi)](https://crates.io/crates/kraapi)
[![Released API docs](https://docs.rs/kraapi/badge.svg)](https://docs.rs/kraapi)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Asynchronous HTTP client for the Kraken cryptocurrency exchange

# Features

- Asynchronous
- Type-driven and self-validating API
- Ergonomic and easy to use
- Promotes re-use of structures and avoids unecessary allocations or redundant HTTP clients

# Installation
Via Crates.io - Add the following to your Cargo.toml:
```
kraapi = "0.2"
```
Via local clone - Add the following to your Cargo.toml
```
kraapi = { path = "path/to_local_checkout" }
```

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
use kraapi::api::AssetPairInfo;
use kraapi::public::KIAssetPairs;
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
- Endpoints that allow a list of some items (assets, asset pairs, transaction IDs, etc.) will
  have methods with the following characteristics:
   - Methods such as `with_asset(...)` or `with_asset_list(...)` always
    **append** to the list. Chained calls to `with_asset(...)` is functionally equivalent to one call
    to `with_asset_list(...)` with the same list of assets
  - Methods such as `update_transaction_list(...)` will always **overwrite** the current data with 
    the new data
  - For endpoints not requiring their list to be populated, methods such as
    `clear_asset_list()` exist to **remove** the previous asset list from the request builder
- The above design allows for templating your requests. You can `clone()` a templated request 
  and then change only the data you care about before sending the request. 
# Examples 
See <https://www.kraken.com/features/api#example-api-code-php-lib> for more info on these
examples

## Public Endpoint - Ticker
```
use kraapi::client::KrakenClient;
use kraapi::public::{KITicker, KOTicker};
use kraapi::api::{KAsset, KAssetPair};

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
use kraapi::client::KrakenClient;
use kraapi::private::{
    KIAddOrder, KOAddOrder};
use kraapi::api::{
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
# P.S.
This library is pronounced "crappy"
