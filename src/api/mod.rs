use std::fmt;
use indexmap::map::IndexMap;
use std::fmt::{Debug,Display};

pub mod public;
pub mod private;

// TODO: Query AssetInfo endpoint and write script to fill out the
// enum and trait impl
pub enum KAsset {
    AUD,
    CAD,
    EUR,
    USD,
    XBT,
    XRP,
}

impl std::fmt::Display for KAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KAsset::AUD => write!(f, "{}", "AUD"),
            KAsset::CAD => write!(f, "{}", "CAD"),
            KAsset::EUR => write!(f, "{}", "EUR"),
            KAsset::USD => write!(f, "{}", "USD"),
            KAsset::XBT => write!(f, "{}", "XBT"),
            KAsset::XRP => write!(f, "{}", "XRP"),
        }
    }
}

impl Debug for KAsset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string().get(1..).unwrap())
    }
}

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
            MethodType::Private => write!(f, "{}", "private"),
            MethodType::Public => write!(f, "{}", "public"),
        }
    }
}

// Used for the AssetPairs endpoint
pub enum AssetPairInfo {
    Info,
    Leverage,
    Fees,
    Margin,
}

impl fmt::Display for AssetPairInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetPairInfo::Info => write!(f, "{}", "info"),
            AssetPairInfo::Leverage => write!(f, "{}", "leverage"),
            AssetPairInfo::Fees => write!(f, "{}", "fees"),
            AssetPairInfo::Margin => write!(f, "{}", "margin"),
        }
    }
}

// Used for the OHLC endpoint
pub enum OHLCInt {
    One,
    Five,
    Fifteen,
    Thirty,
    Sixty,
    TwoForty,
    FourteenForty,
    TenEighty,
    TwentyoneSixty,
}

impl fmt::Display for OHLCInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OHLCInt::One => write!(f, "{}", "1"),
            OHLCInt::Five => write!(f, "{}", "5"),
            OHLCInt::Fifteen => write!(f, "{}", "15"),
            OHLCInt::Thirty => write!(f, "{}", "30"),
            OHLCInt::Sixty => write!(f, "{}", "60"),
            OHLCInt::TwoForty => write!(f, "{}", "240"),
            OHLCInt::FourteenForty => write!(f, "{}", "1440"),
            OHLCInt::TenEighty => write!(f, "{}", "10080"),
            OHLCInt::TwentyoneSixty => write!(f, "{}", "21600"),
        }
    }
}

pub enum OrderCloseTime {
    Open,
    Close,
    Both,
}

impl fmt::Display for OrderCloseTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderCloseTime::Open => write!(f, "{}", "open"),
            OrderCloseTime::Close => write!(f, "{}", "close"),
            OrderCloseTime::Both => write!(f, "{}", "both"),
        }
    }
}

pub enum TradeHistoryType {
    All,
    PosAny,
    PosClosed,
    PosClosing,
    PosNone,
}

impl fmt::Display for TradeHistoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeHistoryType::All => write!(f, "{}", "all"),
            TradeHistoryType::PosAny => write!(f, "{}", "any+position"),
            TradeHistoryType::PosClosed => write!(f, "{}", "closed+position"),
            TradeHistoryType::PosClosing => write!(f, "{}", "closing+position"),
            TradeHistoryType::PosNone => write!(f, "{}", "no+position"),
        }
    }
}

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
            LedgerType::All => write!(f, "{}", "all"),
            LedgerType::Deposit => write!(f, "{}", "deposit"),
            LedgerType::Withdrawal => write!(f, "{}", "withdrawal"),
            LedgerType::Trade => write!(f, "{}", "trade"),
            LedgerType::Margin => write!(f, "{}", "margin"),
        }
    }
}

pub enum TransactionType {
    Buy,
    Sell,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Buy => write!(f, "{}", "buy"),
            TransactionType::Sell => write!(f, "{}", "sell"),
        }
    }
}

pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
    SettlePosition,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Market => write!(f, "{}", "market"),
            OrderType::Limit => write!(f, "{}", "limit"),
            OrderType::StopLoss => write!(f, "{}", "stop-loss"),
            OrderType::TakeProfit => write!(f, "{}", "take-profit"),
            OrderType::StopLossLimit => write!(f, "{}", "stop-loss-limit"),
            OrderType::TakeProfitLimit => write!(f, "{}", "take-profit-limit"),
            OrderType::SettlePosition => write!(f, "{}", "settle-position"),
        }
    }
}

pub(crate) struct EndpointInfo {
    methodtype: MethodType,
    endpoint: String,
}

impl EndpointInfo {
    pub(crate) fn get_type(&self) -> &MethodType {
        &self.methodtype
    }
    pub(crate) fn get_endpoint(&self) -> &String {
        &self.endpoint
    } 
}

pub struct KrakenInput {
    info: EndpointInfo,
    params: Option<IndexMap<String, String>>
}

impl KrakenInput {
    pub(crate) fn get_info(&self) -> &EndpointInfo {
        &self.info
    }

    pub(crate) fn get_params(&self) -> Option<&IndexMap<String, String>> {
        match &self.params {
            Some(params) => Some(&params),
            None => None
        }
    }
}

pub trait Input {
    fn finish(self) -> KrakenInput;
    fn finish_clone(self) -> (KrakenInput, Self);
}

// Module privatemod is needed to prevent leakage into the public API
pub(crate) mod privatemod {
    use indexmap::map::IndexMap;
    // This trait allows us to get a mutable reference to the input data 
    pub trait MutateInput {
        // Get mutable access to the input parameters of the implementing type
        fn list_mut(&mut self) -> &mut IndexMap<String, String>;
    }

    // Trait Inheritance from MutateInput. Everything that implements IntoInputList also needs
    // to implement MutateInput but MutateInput needs to be able to be implmented on types not
    // implementing IntoInputList or its children
    pub trait IntoInputList : MutateInput {
        // Resolve the name of the key associated with the given list
        // Allows to be generic over asset lists, asset pair lists, etc.
        fn list_name(&self) -> String; 
    }
}

// This trait is used in the public API to expose a method for adding a single key value pair.
// Not all endpoints allow a list of items even though the two endpoints share the item type, so we
// need to present two traits to allow/disallow lists of items for each unique endpoint
// ListItem is some type that we want to format like above (assets, assets pairs, transaction ids,
// ledger ids)
pub trait InputListItem : privatemod::IntoInputList {
    type ListItem;

    fn for_item(mut self, item: Self::ListItem) -> Self 
        where Self: Sized,
              Self::ListItem: Display,
    {
        self.format_item(item);
        self
    }

    fn format_item(&mut self, item: Self::ListItem) 
        where Self::ListItem: Display
    {
        let listname = self.list_name();
        match self.list_mut().get_mut(&listname) {
            Some(list) => {
                // Silently disallow adding the same input to the list multiple times
                if list.contains(&item.to_string()) {
                    return;
                }

                *list = format!("{},{}", list, item.to_string());
            },
            None => {
                self.list_mut().insert(listname, item.to_string());
            },
        }
    }
}

// Fun stuff. If there exists a list of items (previously called for_item()), then iterate
// over the list and comma separate the items. If no list exists before calling for_item_list(),
// first consume the first item and then recursivly consume the rest. Note the recursion consumes self 
// and is equivalent to chaining calls to for_item()
// for_item_list is just syntactic sugar for chaining calls to for_item(). Alternating calls to
// either method would also work since they would just concatenate a list item
pub trait InputList : InputListItem {
    fn for_item_list<U>(mut self, items: U) -> Self
        where U: IntoIterator<Item = Self::ListItem>,
              Self: Sized,
              Self::ListItem: Display,
    {
        let listname = self.list_name();
        match self.list_mut().contains_key(&listname) {
            true => {
                items.into_iter().for_each(|item| self.format_item(item));
                self
            },
            false => {
                let mut iter = items.into_iter();
                match iter.next() {
                    Some(val) => {
                        self.list_mut().insert(listname, val.to_string());
                        self.for_item_list(iter)
                    },
                    None => self,
                }
            }
        }
    }
}

// This trait works somwehat similar to InputListItem but the key difference is successive calls
// into InputListItem::for_item() will always concatenate the value to the end of a comma delimited
// array whereas UpdateInput will always overwrite the previous value or create a new key value
// pair if the key doesn't exist yet
// This is a crate internal
pub(crate) trait UpdateInput : privatemod::MutateInput {
    fn update_item<T>(mut self, key: &str, value: T) -> Self 
        where Self: Sized,
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
    where T: Display,
          U: Display
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
        },
        None => None
    }
}
