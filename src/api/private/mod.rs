use http::method::Method;
use hyper::client::ResponseFuture;
use hyper::header::{CONTENT_TYPE, USER_AGENT};
use hyper::{Client, Request, Body};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug,Display};
use indexmap::map::IndexMap;

use crate::auth::KrakenAuth;
use crate::client::KrakenClient;

#[derive(Deserialize, Serialize, Debug)]
pub struct TradeBalance {
    /// equivalent balance (combined balance of all currencies)
    pub eb: String,
    /// trade balance (combined balance of all equity currencies)
    pub tb: String,
    /// margin amount of open positions
    pub m: String,
    /// unrealized net profit/loss of open positions
    pub n: String,
    /// cost basis of open positions
    pub c: String,
    /// current floating valuation of open positions
    pub v: String,
    /// equity = trade balance + unrealized net profit/loss
    pub e: String,
    /// free margin = equity - initial margin (maximum margin available to open new positions)
    pub mf: String,
    /// margin level = (equity / initial margin) * 100
    pub ml: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderDescription {
    pub leverage: String,
    pub order: String,
    pub ordertype: String,
    pub pair: String,
    pub price: String,
    pub price2: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// order pending book entry
    Pending,
    /// open order
    Open,
    /// closed order
    Closed,
    /// order canceled
    Canceled,
    /// order expired
    Expired,
}

/// General order info object
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderInfo {
    /// unix timestamp of when order was closed
    pub closetm: Option<f64>,
    /// total cost (quote currency unless unless viqc set in oflags)
    pub cost: String,
    pub descr: OrderDescription,
    /// unix timestamp of order end time (or 0 if not set)
    pub expiretm: f64,
    /// total fee (quote currency)
    pub fee: String,
    /// comma delimited list of miscellaneous info:
    /// + stopped = triggered by stop price
    /// + touched = triggered by touch price
    /// + liquidated = liquidation
    /// + partial = partial fill
    pub misc: String,
    /// comma delimited list of order flags:
    /// + viqc = volume in quote currency
    /// + fcib = prefer fee in base currency (default if selling)
    /// + fciq = prefer fee in quote currency (default if buying)
    /// + nompp = no market price protection
    pub oflags: String,
    /// unix timestamp of when order was placed
    pub opentm: f64,
    /// average price (quote currency unless viqc set in oflags)
    pub price: String,
    /// stop price (quote currency, for trailing stops)
    pub stopprice: Option<String>,
    /// triggered limit price (quote currency, when limit based order type triggered)
    pub limitprice: Option<String>,
    /// additional info on status (if any)
    pub reason: Option<String>,
    /// Referral order transaction id that created this order
    pub refid: Option<String>,
    /// unix timestamp of order start time (or 0 if not set)
    pub starttm: f64,
    /// status of order:
    pub status: OrderStatus,
    /// user reference id
    pub userref: Option<String>,
    /// volume of order (base currency unless viqc set in oflags)
    pub vol: String,
    /// volume executed (base currency unless viqc set in oflags)
    pub vol_exec: String,
}

/// Open orders
#[derive(Deserialize, Serialize, Debug)]
pub struct OpenOrders {
    pub open: HashMap<String, OrderInfo>,
}

/// Closed order result
#[derive(Deserialize, Serialize, Debug)]
pub struct ClosedOrders {
    pub closed: HashMap<String, OrderInfo>,
    pub count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ClosedOrdersConfigCloseTime {
    Open,
    Close,
    Both,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClosedOrdersConfig {
    /// whether or not to include trades in output (optional.  default = false).
    pub trades: Option<bool>,
    /// restrict results to given user reference id (optional).
    pub userref: Option<String>,
    /// starting unix timestamp or order tx id of results (optional.  exclusive).
    pub start: Option<i64>,
    /// ending unix timestamp or order tx id of results (optional.  inclusive).
    pub end: Option<i64>,
    /// result offset.
    pub ofs: Option<u64>,
    /// which time to use (optional).
    pub closetime: Option<ClosedOrdersConfigCloseTime>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TradeType {
    Buy,
    Sell,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderType {
    Market,
    /// (price = limit price)
    Limit,
    /// (price = stop loss price)
    StopLoss,
    /// (price = take profit price)
    TakeProfit,
    /// (price = stop loss price, price2 = take profit price)
    StopLossProfit,
    /// (price = stop loss price, price2 = take profit price)
    StopLossProfitLimit,
    /// (price = stop loss trigger price, price2 = triggered limit price)
    StopLossLimit,
    /// (price = take profit trigger price, price2 = triggered limit price)
    TakeProfitLimit,
    /// (price = trailing stop offset)
    TrailingStop,
    /// (price = trailing stop offset, price2 = triggered limit offset)
    TrailingStopLimit,
    /// (price = stop loss price, price2 = limit price)
    StopLossAndLimit,
    SettlePosition,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewOrder {
    /// asset pair
    pub pair: String,
    /// type of order (buy/sell)
    pub kind: TradeType,
    pub order_type: OrderType,
    /// price (optional.  dependent upon ordertype)
    pub price: Option<String>,
    /// secondary price (optional.  dependent upon ordertype)
    pub price2: Option<String>,
    /// order volume in lots
    pub volume: String,
    /// amount of leverage desired (optional.  default = none)
    pub leverage: Option<String>,
    /// comma delimited list of order flags (optional):
    ///   + viqc = volume in quote currency (not available for leveraged orders)
    ///   + fcib = prefer fee in base currency
    ///   + fciq = prefer fee in quote currency
    ///   + nompp = no market price protection
    ///   + post = post only order (available when ordertype = limit)
    pub oflags: Option<String>,
    /// scheduled start time (optional):
    ///   + 0 = now (default)
    ///   + +<n> = schedule start time <n> seconds from now
    ///   + <n> = unix timestamp of start time
    pub starttm: Option<i64>,
    /// expiration time (optional):
    ///   + 0 = no expiration (default)
    ///   + +<n> = expire <n> seconds from now
    ///   + <n> = unix timestamp of expiration time
    pub expiretm: Option<i64>,
    /// user reference id.  32-bit signed number.  (optional)
    pub userref: Option<String>,
    /// validate inputs only.  do not submit order (optional)
    pub validate: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CanceldOrders {
    /// number of orders canceled
    count: u32,
    /// if set, order(s) is/are pending cancellation
    pending: u32,
}

pub fn format_params<T, U>(params: &IndexMap<T, U>) -> String
    where T: Display,
          U: Display
{
    let mut res = String::new();
    for index in 0..params.len() {
        let pair = params.get_index(index).unwrap();
        if index == 0 {
            res = format!("{}{}={}", res, pair.0, pair.1);
        } else {
            res = format!("{}&{}={}", res, pair.0, pair.1);
        }
    }
    
    return res;
}

pub fn get_server_time(client: &KrakenClient) -> ResponseFuture {
    let mut time = Request::builder()
        .method("GET")
        .uri("https://api.kraken.com/0/public/Time")
        .body(Body::empty())
        .unwrap();
    client.request(time)
}

pub fn get_system_status(client: &KrakenClient) -> ResponseFuture {
    let mut status = Request::builder()
        .method("GET")
        .uri("https://api.kraken.com/0/public/SystemStatus")
        .body(Body::empty())
        .unwrap();
    client.request(status)
}

pub fn get_account_balance(client: &KrakenClient) -> ResponseFuture {
    let nonce = KrakenAuth::nonce();
    println!("{:?}", nonce);
    let endpoint = format!("/{}/{}/{}", client.get_version(), "private", "Balance");
    let mut params = IndexMap::new();
    params.insert("nonce", &nonce);
    let format_params = format_params(&params);
    let signature = client.get_auth().sign(&endpoint, &nonce, &format_params);
    let full_url = format!("{}{}", client.get_url(), endpoint);

    let mut request = Request::builder()
        .method("POST")
        .uri(full_url)
        .body(Body::from(format_params))
        .expect("Failed to form a correct http request");

    request.headers_mut().insert(USER_AGENT, "krakenapi/0.1 (Kraken Rust Client)".parse().unwrap());
    request.headers_mut().insert("API-Key", client.get_auth().get_key().parse().unwrap());
    request.headers_mut().insert("API-Sign", signature.parse().unwrap());
    request.headers_mut().insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());

    println!("{:?}", request);

    client.request(request)
}

pub fn get_trade_balance(client: &KrakenClient) -> ResponseFuture {
    let nonce = KrakenAuth::nonce();
    let asset = String::from("xbt");
    println!("{:?}", nonce);
    let endpoint = format!("/{}/{}/{}", client.get_version(), "private", "TradeBalance");
    let mut params = IndexMap::new();
    params.insert("nonce", &nonce);
    params.insert("asset", &asset);
    let format_params = format_params(&params);
    let signature = client.get_auth().sign(&endpoint, &nonce, &format_params);
    let full_url = format!("{}{}", client.get_url(), endpoint);

    let mut request = Request::builder()
        .method("POST")
        .uri(full_url)
        .body(Body::from(format_params))
        .expect("Failed to form a correct http request");
    
    request.headers_mut().insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    request.headers_mut().insert(USER_AGENT, "krakenapi/0.1 (Kraken Rust Client)".parse().unwrap());
    request.headers_mut().insert("API-Key", client.get_auth().get_key().parse().unwrap());
    request.headers_mut().insert("API-Sign", signature.parse().unwrap());

    println!("{:?}", request);

    client.request(request)
}

pub fn get_trade_volume(client: &KrakenClient) -> ResponseFuture {
    let nonce = KrakenAuth::nonce();
    println!("{:?}", nonce);
    let endpoint = format!("/{}/{}/{}", client.get_version(), "private", "TradeVolume");
    let xbt = "xbtusd".to_string();
    let mut params = IndexMap::new();
    params.insert("nonce", &nonce);
    params.insert("pair", &xbt);
    let format_params = format_params(&params);
    let signature = client.get_auth().sign(&endpoint, &nonce, &format_params);
    let full_url = format!("{}{}", client.get_url(), endpoint);

    let mut request = Request::builder()
        .method("POST")
        .uri(full_url)
        .body(Body::from(format_params))
        .expect("Failed to form a correct http request");
    
    request.headers_mut().insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    request.headers_mut().insert(USER_AGENT, "krakenapi/0.1 (Kraken Rust Client)".parse().unwrap());
    request.headers_mut().insert("API-Key", client.get_auth().get_key().parse().unwrap());
    request.headers_mut().insert("API-Sign", signature.parse().unwrap());

    println!("{:?}", request);

    client.request(request)
}
