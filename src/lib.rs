//! # Kraapi
//!
//! Asynchronous HTTP client for the Kraken cryptocurrency exchange
//!
//! # Features
//!
//! - Asynchronous
//! - Type-driven and self-validating API
//! - Ergonomic and easy to use
//! - Promotes re-use of structures and avoids unecessary allocations or redundant HTTP clients
//!
//! # General Notes - TLDR
//!
//! - Every [input][api::Input] type is prefixed with KI. Every [output][api::Output]
//!   type is prefixed with KO
//! - Every [input][api::Input] type is a builder type for [KrakenInput][api::KrakenInput].
//!   All required paramters(per Kraken) are parameters for that type's constructor.
//!   Optional parameters are exposed using methods.
//! - Only a [KrakenInput][api::KrakenInput] instance can be passed into the
//!   [KrakenClient][client::KrakenClient]. You must fufill any contracts exposed by the
//!   type and convert it to a [KrakenInput][api::KrakenInput] by calling
//!   [finish()][api::Input::finish] or [finish_clone()][api::Input::finish_clone]
//!   which exist for every [input][api::Input] type
//! - You must await the call to request
//! - Deserializing the data returned from Kraken into output structs is done for you. Currently if
//!   you pass in the wrong [ouput][api::Output] type, the parsing will fail
//! - Builder methods require ownership so if you must perform some application logic while
//!   building a [KrakenInput][api::KrakenInput] you must reassign the variable like so:
//!
//! ```
//! # use kraapi::api::asset::AssetPairInfo;
//! # use kraapi::public::asset_pairs::KIAssetPairs;
//! # use kraapi::api::Input;
//! let some_application_logic = true;
//! // mut to allow reassignment based on application logic
//! let mut input = KIAssetPairs::build();
//!
//! if some_application_logic {
//!     input = input.info(AssetPairInfo::Leverage);
//! } else {
//!     input = input.info(AssetPairInfo::Margin);
//! }
//!
//! // Now of type KrakenInput so we have to rebind the variable
//! let input = input.finish();
//! ```
//! - Endpoints that allow a list of some items (assets, asset pairs, transaction IDs, etc.) will
//!   have methods with the following characteristics:
//!   - Methods such as `with_asset(...)` or `with_asset_list(...)` always
//!     **append** to the list. Chained calls to `with_asset(...)` is functionally equivalent to one call
//!     to `with_asset_list(...)` with the same list of assets
//!   - Methods such as `update_transaction_list(...)` will always **overwrite** the current data with 
//!     the new data
//!   - For endpoints not requiring their list to be populated, methods such as
//!     `clear_asset_list()` exist to **remove** the previous asset list from the request builder
//! - The above design allows for templating your requests. You can `clone()` a templated request 
//!   and then change only the data you care about before sending the request. 
//! # Examples
//! See <https://www.kraken.com/features/api#example-api-code-php-lib> for more info on these
//! examples
//!
//! ## Public Endpoint - Ticker
//! ```
//! use kraapi::client::KrakenClient;
//! use kraapi::public::ticker::{KITicker, KOTicker};
//! use kraapi::api::asset::{KAsset, KAssetPair};
//! use kraapi::api::Input;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KrakenClient::new("", "");
//!
//! let ticker_input = KITicker::build(KAssetPair(KAsset::XBT, KAsset::USD)).finish();
//!
//! let ticker_output = client.request::<KOTicker>(&ticker_input).await?;
//!
//! println!("{:#?}", ticker_output);
//! # Ok(())
//! # }
//! ```
//! ## Private Endpoint - Add Order
//! ```
//! use kraapi::client::KrakenClient;
//! use kraapi::private::add_order::{
//!     KIAddOrder, KOAddOrder, Leverage};
//! use kraapi::api::{
//!     asset::{KAsset, KAssetPair},
//!     TradeType, OrderType};
//! use kraapi::api::Input;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Example credentials from Kraken's documentation. Personal credentials will be needed for
//! // private API endpoints
//! let client = KrakenClient::new(
//!     "<Your_API_Key>",
//!     "<Your_API_Secret>"
//!     );
//!
//! let add_order_input = KIAddOrder::build(
//!     KAssetPair(KAsset::XBT, KAsset::USD),
//!     TradeType::Buy,
//!     OrderType::Limit(String::from("101.9901")),
//!     2.12345678)
//!     .with_leverage(Leverage::Two)
//!     .with_closing_order(OrderType::StopLossLimit(String::from("#5%"), String::from("#10")))
//!     .validate(true)
//!     .finish();
//!
//! // Valid credentials to be entered above, otherwise this will panic
//! // let add_order_output = client.request::<KOAddOrder>(&add_order_input).await?;
//! # let add_order_output = String::from("");
//!
//! println!("{:#?}", add_order_output);
//! # Ok(())
//! # }
//! ```
//!
//! # P.S.
//!
//! This library is pronounced "crappy"

pub mod api;
mod auth;
pub mod client;
pub mod error;

pub use api::private;
pub use api::public;
