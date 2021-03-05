//! Module encapsulating the private and public API endpoints of the Kraken exchange

use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display};

use super::error::KrakenError;

pub mod private;
pub mod public;

/// Result alias. Either contains the output struct of some type `T` that implements [Output]
/// or a vector of [KrakenError][super::error::KrakenError]`s
pub type KrakenResult<T> = Result<T, Vec<KrakenError>>;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct KResult<T> {
    /// Generic payload type. T will be some type prefixed with KO
    pub result: Option<T>,
    /// Vector of zero or more errors returned from Kraken
    pub error: Vec<String>,
}

// TODO: Query AssetInfo endpoint and write script to fill out the
// enum and trait impl
/// Assets accepted on the Kraken Exchange
/// # FIXME
/// Basic currencies used for testing. Open pull request to add more currencies <https://github.com/Fuzzy-Math/KrakenAPI-Rust>
pub enum KAsset {
    /// Australian Dollar
    AUD,
    /// Canadian Dollar
    CAD,
    /// Euro
    EUR,
    /// United States Dollar
    USD,
    /// Bitcoin
    XBT,
    /// Ripple
    XRP,
}

impl std::fmt::Display for KAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KAsset::AUD => write!(f, "AUD"),
            KAsset::CAD => write!(f, "CAD"),
            KAsset::EUR => write!(f, "EUR"),
            KAsset::USD => write!(f, "USD"),
            KAsset::XBT => write!(f, "XBT"),
            KAsset::XRP => write!(f, "XRP"),
        }
    }
}

impl Debug for KAsset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string().get(1..).unwrap())
    }
}

/// Tradeable asset pair
/// # FIXME
/// Kraken only accepts certain asset pairs as listed from the asset pair endpoint.
/// This data probably should be parsed into a lookup table to ensure only support pairs are
/// accepted.
/// Open a pull request at <https://github.com/Fuzzy-Math/KrakenAPI-Rust>
pub struct KAssetPair(pub KAsset, pub KAsset);

impl fmt::Display for KAssetPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0.to_string(), self.1.to_string())
    }
}

pub(crate) enum MethodType {
    Private,
    Public,
}

impl fmt::Display for MethodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MethodType::Private => write!(f, "private"),
            MethodType::Public => write!(f, "public"),
        }
    }
}

/// System status | See [KISystemStatus][public::system_status::KISystemStatus]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SystemStatus {
    /// Operational, full trading available
    Online,
    /// Existing orders are cancelable, but new orders cannot be created
    CancelOnly,
    /// Existing orders are cancelable, and only new post limit orders can be submitted
    PostOnly,
    /// Existing orders are cancelable, and only new limit orders can be submitted
    LimitOnly,
    /// System is offline for maintenance
    Offline,
}

/// Asset pair info to retreive | See [KIAssetPairs][public::asset_pairs::KIAssetPairs]
pub enum AssetPairInfo {
    Info,
    Leverage,
    Fees,
    Margin,
}

impl fmt::Display for AssetPairInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetPairInfo::Info => write!(f, "info"),
            AssetPairInfo::Leverage => write!(f, "leverage"),
            AssetPairInfo::Fees => write!(f, "fees"),
            AssetPairInfo::Margin => write!(f, "margin"),
        }
    }
}

/// OHLC time frame interval in minutes | See [KIOHLC][public::ohlc::KIOHLC]
pub enum OHLCInterval {
    /// 1 minute
    One,
    /// 5 minutes
    Five,
    /// 15 minutes
    Fifteen,
    /// 30 minutes
    Thirty,
    /// 1 hour
    Sixty,
    /// 4 hours
    TwoForty,
    /// 1 day (24 hours)
    FourteenForty,
    /// 1 week (7 days)
    TenEighty,
    /// ~2 weeks (15 days)
    TwentyoneSixty,
}

impl fmt::Display for OHLCInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OHLCInterval::One => write!(f, "1"),
            OHLCInterval::Five => write!(f, "5"),
            OHLCInterval::Fifteen => write!(f, "15"),
            OHLCInterval::Thirty => write!(f, "30"),
            OHLCInterval::Sixty => write!(f, "60"),
            OHLCInterval::TwoForty => write!(f, "240"),
            OHLCInterval::FourteenForty => write!(f, "1440"),
            OHLCInterval::TenEighty => write!(f, "10080"),
            OHLCInterval::TwentyoneSixty => write!(f, "21600"),
        }
    }
}

/// See [KIClosedOrders][private::closed_orders::KIClosedOrders]
pub enum OrderCloseTime {
    Open,
    Close,
    Both,
}

impl fmt::Display for OrderCloseTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderCloseTime::Open => write!(f, "open"),
            OrderCloseTime::Close => write!(f, "close"),
            OrderCloseTime::Both => write!(f, "both"),
        }
    }
}

/// Type of trade to query history for | See [KITradeHistory][private::trade_history::KITradeHistory]
pub enum TradeHistoryType {
    /// All types
    All,
    /// Any position (open or closed)
    PosAny,
    /// Closed positions
    PosClosed,
    /// Trades closing all or part of a position
    PosClosing,
    /// Non-positional trades
    PosNone,
}

impl fmt::Display for TradeHistoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeHistoryType::All => write!(f, "all"),
            TradeHistoryType::PosAny => write!(f, "any+position"),
            TradeHistoryType::PosClosed => write!(f, "closed+position"),
            TradeHistoryType::PosClosing => write!(f, "closing+position"),
            TradeHistoryType::PosNone => write!(f, "no+position"),
        }
    }
}

/// Ledger type to retrieve | See [KILedgerInfo][private::ledger_info::KILedgerInfo]
pub enum LedgerType {
    All,
    Deposit,
    Withdrawal,
    Trade,
    Margin,
}

impl fmt::Display for LedgerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LedgerType::All => write!(f, "all"),
            LedgerType::Deposit => write!(f, "deposit"),
            LedgerType::Withdrawal => write!(f, "withdrawal"),
            LedgerType::Trade => write!(f, "trade"),
            LedgerType::Margin => write!(f, "margin"),
        }
    }
}

/// Order trade type | See [KIAddOrder][private::add_order::KIAddOrder]
pub enum TradeType {
    Buy,
    Sell,
}

impl fmt::Display for TradeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeType::Buy => write!(f, "buy"),
            TradeType::Sell => write!(f, "sell"),
        }
    }
}

/// Order Type | See [KIAddOrder][private::add_order::KIAddOrder]
///
/// Prices can be preceded by +, -, or # to signify the price as a relative amount
/// (with the exception of trailing stops, which are always relative). + adds the amount
/// to the current offered price. - subtracts the amount from the current offered price.
/// # will either add or subtract the amount to the current offered price, depending on
/// the type and order type used. Relative prices can be suffixed with a % to signify
/// the relative amount as a percentage of the offered price.
pub enum OrderType {
    /// Market order type with market price inferred
    Market,
    /// Limit order along with the limit price
    Limit(String),
    /// Stop Loss order with the stop loss price
    StopLoss(String),
    /// Take Profit order with the take profit price
    TakeProfit(String),
    /// Stop Loss Limit order with the stop loss trigger price and the triggered limit price
    StopLossLimit(String, String),
    /// Take Profit Limit order with the take profit trigger price and the triggered limit price
    TakeProfitLimit(String, String),
    SettlePosition,
}

use OrderType::{
    Limit, Market, SettlePosition, StopLoss, StopLossLimit, TakeProfit, TakeProfitLimit,
};
impl OrderType {
    // FIXME: Avoid the empty strings using options and fix the pattern matching in
    // percent_encode()
    fn elide(&self) -> (Option<String>, Option<String>) {
        match self {
            Market => (None, None),
            Limit(price1) => (Some(price1.to_string()), None),
            StopLoss(price1) => (Some(price1.to_string()), None),
            TakeProfit(price1) => (Some(price1.to_string()), None),
            StopLossLimit(price1, price2) => (Some(price1.to_string()), Some(price2.to_string())),
            TakeProfitLimit(price1, price2) => (Some(price1.to_string()), Some(price2.to_string())),
            SettlePosition => (None, None),
        }
    }

    pub(crate) fn percent_encode(&self) -> (Option<String>, Option<String>) {
        match self.elide() {
            (Some(price1), Some(price2)) => {
                let encoded_price1 = price1
                    .replace("+", "%2B")
                    .replace("#", "%23")
                    .replace("%", "%25");

                let encoded_price2 = price2
                    .replace("+", "%2B")
                    .replace("#", "%23")
                    .replace("%", "%25");

                (Some(encoded_price1), Some(encoded_price2))
            }
            (Some(price1), None) => {
                let encoded_price1 = price1
                    .replace("+", "%2B")
                    .replace("#", "%23")
                    .replace("%", "%25");

                (Some(encoded_price1), None)
            }
            (None, Some(_)) => {
                unreachable!()
            }
            (None, None) => (None, None),
        }
    }

    pub(crate) fn price1(&self) -> Option<String> {
        match self.percent_encode() {
            (Some(price), _) => Some(price),
            (None, _) => None,
        }
    }

    pub(crate) fn price2(&self) -> Option<String> {
        match self.percent_encode() {
            (_, Some(price)) => Some(price),
            (_, None) => None,
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Market => write!(f, "market"),
            OrderType::Limit(_) => write!(f, "limit"),
            OrderType::StopLoss(_) => write!(f, "stop-loss"),
            OrderType::TakeProfit(_) => write!(f, "take-profit"),
            OrderType::StopLossLimit(_, _) => write!(f, "stop-loss-limit"),
            OrderType::TakeProfitLimit(_, _) => write!(f, "take-profit-limit"),
            OrderType::SettlePosition => write!(f, "settle-position"),
        }
    }
}

/// Add order flags | See [KIAddOrder][private::add_order::KIAddOrder]
pub enum OrderFlags {
    /// Prefer fee in base currency
    BaseCurrency,
    /// Prefer fee in quote currency
    QuoteCurrency,
    /// No market price protection
    NoMarketPriceProtection,
    /// Post only order (when ordertype is limit)
    PostOnly,
}

impl fmt::Display for OrderFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderFlags::BaseCurrency => write!(f, "fcib"),
            OrderFlags::QuoteCurrency => write!(f, "fciq"),
            OrderFlags::NoMarketPriceProtection => write!(f, "nompp"),
            OrderFlags::PostOnly => write!(f, "post"),
        }
    }
}

pub(crate) struct EndpointInfo {
    methodtype: MethodType,
    endpoint: String,
}

impl EndpointInfo {
    pub(crate) fn method(&self) -> &MethodType {
        &self.methodtype
    }
    pub(crate) fn endpoint(&self) -> &String {
        &self.endpoint
    }
}

/// Fully constructed input data to be passed to a [KrakenClient][super::client::KrakenClient]
/// # Note
/// KrakenInput can't be constructed directly. An instance is created by calling finish() or
/// finish_clone() on an input builder type (types prefixed with "KI"). See the [Input] trait for
/// KrakenInput builder types
pub struct KrakenInput {
    info: EndpointInfo,
    params: Option<IndexMap<String, String>>,
}

impl KrakenInput {
    pub(crate) fn info(&self) -> &EndpointInfo {
        &self.info
    }

    pub(crate) fn params(&self) -> Option<&IndexMap<String, String>> {
        match &self.params {
            Some(params) => Some(&params),
            None => None,
        }
    }
}

/// Trait used by input builder types to construct a [KrakenInput]. All input builder
/// types implement this trait
pub trait Input {
    fn finish(self) -> KrakenInput;
    fn finish_clone(self) -> (KrakenInput, Self);
}

/// Marker trait for output types that are returned from Kraken. Ensures that
/// [KrakenClient][super::client::KrakenClient]'s [request][super::client::KrakenClient::request]
/// method expects the correct output types
pub trait Output {}

// This trait allows us to get a mutable reference to the input data
pub(crate) trait MutateInput {
    // Get mutable access to the input parameters of the implementing type
    fn list_mut(&mut self) -> &mut IndexMap<String, String>;
}

// Trait Inheritance from MutateInput. Everything that implements IntoInputList also needs
// to implement MutateInput but MutateInput needs to be able to be implmented on types not
// implementing IntoInputList or its children
pub(crate) trait IntoInputList: MutateInput {
    // Resolve the name of the key associated with the given list
    // Allows to be generic over asset lists, asset pair lists, etc.
    fn list_name(&self) -> String;
}

// This trait is used in the public API to expose a method for adding a single key value pair.
// Not all endpoints allow a list of items even though the two endpoints share the item type, so we
// need to present two traits to allow/disallow lists of items for each unique endpoint
// ListItem is some type that we want to format like above (assets, assets pairs, transaction ids,
// ledger ids)
pub(crate) trait InputListItem: IntoInputList {
    type ListItem;

    fn with_item(mut self, item: Self::ListItem) -> Self
    where
        Self: Sized,
        Self::ListItem: Display,
    {
        self.format_item(item);
        self
    }

    fn format_item(&mut self, item: Self::ListItem)
    where
        Self::ListItem: Display,
    {
        let listname = self.list_name();
        match self.list_mut().get_mut(&listname) {
            Some(list) => {
                // Silently disallow adding the same input to the list multiple times
                if list.contains(&item.to_string()) {
                    return;
                }

                *list = format!("{},{}", list, item.to_string());
            }
            None => {
                self.list_mut().insert(listname, item.to_string());
            }
        }
    }
}

// Fun stuff. If there exists a list of items (previously called with_item()), then iterate
// over the list and comma separate the items. If no list exists before calling with_item_list(),
// first consume the first item and then recursivly consume the rest. Note the recursion consumes self
// and is equivalent to chaining calls to with_item()
// with_item_list is just syntactic sugar for chaining calls to with_item(). Alternating calls to
// either method would also work since they would just concatenate a list item
pub(crate) trait InputList: InputListItem {
    fn with_item_list<U>(mut self, items: U) -> Self
    where
        U: IntoIterator<Item = Self::ListItem>,
        Self: Sized,
        Self::ListItem: Display,
    {
        let listname = self.list_name();
        match self.list_mut().contains_key(&listname) {
            true => {
                items.into_iter().for_each(|item| self.format_item(item));
                self
            }
            false => {
                let mut iter = items.into_iter();
                match iter.next() {
                    Some(val) => {
                        self.list_mut().insert(listname, val.to_string());
                        self.with_item_list(iter)
                    }
                    None => self,
                }
            }
        }
    }
}

// This trait works somwehat similar to InputListItem but the key difference is successive calls
// into InputListItem::with_item() will always concatenate the value to the end of a comma delimited
// array whereas UpdateInput will always overwrite the previous value or create a new key value
// pair if the key doesn't exist yet
pub(crate) trait UpdateInput: MutateInput {
    fn update_input<T>(mut self, key: &str, value: T) -> Self
    where
        Self: Sized,
        T: Display,
    {
        match self.list_mut().get_mut(key) {
            Some(key) => {
                *key = value.to_string();
                self
            }
            None => {
                self.list_mut().insert(String::from(key), value.to_string());
                self
            }
        }
    }
}

pub(crate) fn format_params<T, U>(params: &Option<&IndexMap<T, U>>) -> Option<String>
where
    T: Display,
    U: Display,
{
    match params {
        Some(params) => {
            let mut res = String::new();
            for index in 0..params.len() {
                let pair = params.get_index(index).unwrap();
                if index == 0 {
                    res = format!("{}{}={}", res, pair.0, pair.1);
                } else {
                    res = format!("{}&{}={}", res, pair.0, pair.1);
                }
            }
            Some(res)
        }
        None => None,
    }
}
