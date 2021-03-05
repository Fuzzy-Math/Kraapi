//! Module for interacting with Kraken's private API endpoints
//! # Note
//! Each type prefixed with "KI" is a [KrakenInput][super::KrakenInput] builder which will build requests for the given
//! endpoint.
//! Each type postfixed with "KO" is the output object that has been returned from Kraken's servers
//! and has been parsed into the given structure.
//! A valid api key and api secret will have to be used when creating a
//! [KrakenClient][super::super::client::KrakenClient] otherwise requests sent to 
//! private endpoints will panic before being sent to Kraken

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Structs/Enums
use super::{
    EndpointInfo, KAsset, KAssetPair, KrakenInput, LedgerType, MethodType, OrderCloseTime,
    OrderFlags, OrderType, TradeHistoryType, TradeType,
};

// Traits
use super::{Input, InputList, InputListItem, IntoInputList, MutateInput, UpdateInput};

/// Get account balance endpoint
pub mod account_balance;

/// Get trade balance endpoint
pub mod trade_balance;

/// Get open orders endpoint
pub mod open_orders;

/// Get closed orders endpoint
pub mod closed_orders;

/// Query orders info endpoint
pub mod query_orders;

/// Get trade history endpoint
pub mod trade_history;

/// Query trades info endpoint
pub mod query_trades;

/// Get open positions endpoint
pub mod open_positions;

/// Get ledgers info endpoint
pub mod ledger_info;

/// Query ledgers endpoint
pub mod query_ledgers;

/// Get trade volume endpoint
pub mod trade_volume;

/// Add standard order endpoint
pub mod add_order;

/// Cancel open order endpoint
pub mod cancel_order;

/// Cancel all orders endpoint
pub mod cancel_all_orders;

/// Cancel all orders after ... endpoint
pub mod cancel_on_timeout;

/// Order description data | See [KOOrderInfo]
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOrderDescription {
    pub pair: String,
    #[serde(rename = "type")]
    pub tradetype: String,
    pub ordertype: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    #[serde(rename = "order")]
    pub desc: String,
    #[serde(rename = "close")]
    pub closedesc: String,
}

/// Order status data | See [KOOrderInfo]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum KOOrderStatus {
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

/// Order info data | See [KOOpenOrders][open_orders::KOOpenOrders] - 
/// [KOClosedOrders][closed_orders::KOClosedOrders] -
/// [KOQueryOrders][query_orders::KOQueryOrders]
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOrderInfo {
    /// Referral order transaction id that created this order
    pub refid: Option<String>,
    /// user reference id
    pub userref: Option<u32>,
    /// status of order:
    pub status: KOOrderStatus,
    /// unix timestamp of when order was placed
    pub opentm: f64,
    /// unix timestamp of order start time (or 0 if not set)
    pub starttm: f64,
    /// unix timestamp of order end time (or 0 if not set)
    pub expiretm: f64,
    /// order description info
    pub descr: KOOrderDescription,
    /// volume of order (base currency unless viqc set in oflags)
    pub vol: String,
    /// volume executed (base currency unless viqc set in oflags)
    pub vol_exec: String,
    /// total cost (quote currency unless unless viqc set in oflags)
    pub cost: String,
    /// total fee (quote currency)
    pub fee: String,
    /// average price (quote currency unless viqc set in oflags)
    pub price: String,
    /// stop price (quote currency, for trailing stops)
    pub stopprice: Option<String>,
    /// triggered limit price (quote currency, when limit based order type triggered)
    pub limitprice: Option<String>,
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
    /// array of trade ids related to order (if trades info requested and data available)
    pub trades: Option<Vec<String>>,
    /// unix timestamp of when order was closed. Field only present when calling ClosedOrders
    /// endpoint
    pub closetm: Option<f64>,
    /// additional info on status (if any). Field only present when calling ClosedOrders
    pub reason: Option<String>,
}

/// Trade info data | See [KOTradesInfo][query_trades::KOTradesInfo] -
/// [KOTradeHistory][trade_history::KOTradeHistory]
#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradeData {
    /// Order responsible for execution of trade
    pub ordertxid: String,
    pub pair: String,
    pub time: f64,
    #[serde(rename = "type")]
    pub tradetype: String,
    pub ordertype: String,
    pub price: String,
    pub cost: String,
    pub fee: String,
    pub vol: String,
    pub margin: Option<String>,
    pub misc: String,
    pub posstatus: Option<String>,
    pub cprice: Option<String>,
    pub cfee: Option<String>,
    pub cvol: Option<String>,
    pub cmargin: Option<String>,
    pub net: Option<String>,
    pub trades: Option<String>,
}

/// Ledger info data | See [KOLedgers]
#[derive(Deserialize, Serialize, Debug)]
pub struct KOLedgerInfo {
    /// Order responsible for execution of trade
    pub refid: String,
    pub time: f64,
    #[serde(rename = "type")]
    pub ledgertype: String,
    pub aclass: String,
    pub asset: String,
    pub amount: String,
    pub fee: String,
    pub balance: Option<String>,
}

/// Response from the Get Ledgers Info or Query Ledgers endpoints | See
/// [KILedgerInfo][ledger_info::KILedgerInfo] -
/// [KIQueryLedgers][query_ledgers::KIQueryLedgers]
#[derive(Deserialize, Serialize, Debug)]
pub struct KOLedgers {
    /// Map with the ledger ID as the key and the ledger info as the value
    #[serde(flatten)]
    pub ledgers: HashMap<String, KOLedgerInfo>,
}
