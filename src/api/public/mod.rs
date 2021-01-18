use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use indexmap::map::IndexMap;

use crate::api::{AssetPairInfo, Input, KAssetPair, KrakenInput, OHLCInt, MethodType, EndpointInfo,
Asset, AssetList, Pair, PairList};

pub struct KIServerTime();

impl KIServerTime {
    pub fn build() -> KrakenInput {
        let time = KIServerTime();
        time.finish_input()
    }
}
impl Input for KIServerTime {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Time") },
           params: None 
       }
    }
}

pub struct KISystemStatus();

impl KISystemStatus {
    pub fn build() -> KrakenInput {
        let status = KISystemStatus();
        status.finish_input()
    }
}

impl Input for KISystemStatus {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("SystemStatus") },
           params: None
       }
    }
}

pub struct KIAssetInfo {
    pub params: IndexMap<String, String>,
}

impl KIAssetInfo {
    pub fn build() -> Self {
        KIAssetInfo {
            params: IndexMap::new()
        }
    }
}

impl Input for KIAssetInfo {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Assets") },
           params: Some(self.params)
       }
    }
}

impl Asset for KIAssetInfo {
    fn get_list(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl AssetList for KIAssetInfo {}

pub struct KIAssetPairs {
    pub params: IndexMap<String, String>,
}

impl KIAssetPairs {
    pub fn build() -> Self {
        KIAssetPairs {
            params: IndexMap::new()
        }
    }

    pub fn info (mut self, info: AssetPairInfo) -> Self {
        self.params.insert(String::from("info"), info.to_string());
        self
    }
}

impl Input for KIAssetPairs {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("AssetPairs") },
           params: Some(self.params)
       }
    }
}

impl Pair for KIAssetPairs {
    fn get_list(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl PairList for KIAssetPairs {}

pub struct KITicker {
    pub params: IndexMap<String, String>,
}

impl KITicker {
    pub fn build(pair: KAssetPair) -> Self {
        let ticker = KITicker {
            params: IndexMap::new()
        };
        ticker.for_pair(pair)
    }

    pub fn build_with_list<T>(pairs: T) -> KITicker 
        where T: IntoIterator<Item = KAssetPair>,
    {
        let ticker = KITicker {
            params: IndexMap::new()
        };
        ticker.for_pair_list(pairs)
    }
}

impl Input for KITicker {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Ticker") },
           params: Some(self.params)
       }
    }
}

impl Pair for KITicker {
    fn get_list(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl PairList for KITicker {}

pub struct KIOHLC {
    pub params: IndexMap<String, String>,
}

impl KIOHLC {
    pub fn build(pair: KAssetPair) -> Self {
        let ohlc = KIOHLC {
            params: IndexMap::new()
        };
        ohlc.for_pair(pair)
    }

    pub fn with_interval (mut self, interval: OHLCInt) -> Self {
        self.params.insert(String::from("interval"), interval.to_string());
        self
    }

    pub fn since(mut self, id: String) -> Self {
        self.params.insert(String::from("since"), id);
        self
    }
}

impl Input for KIOHLC {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("OHLC") },
           params: Some(self.params)
       }
    }
}

impl Pair for KIOHLC {
    fn get_list(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

pub struct KIOrderBook {
    pub params: IndexMap<String, String>,
}

impl KIOrderBook {
    pub fn build(pair: KAssetPair) -> Self {
        let order_book = KIOrderBook {
            params: IndexMap::new()
        };
        order_book.for_pair(pair)
    }

    pub fn with_max (mut self, max: i64) -> Self {
        self.params.insert(String::from("count"), max.to_string());
        self
    }
}

impl Input for KIOrderBook {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Depth") },
           params: Some(self.params)
       }
    }
}

impl Pair for KIOrderBook {
    fn get_list(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

pub struct KIRecentTrades {
    pub params: IndexMap<String, String>,
}

impl KIRecentTrades {
    pub fn build(pair: KAssetPair) -> Self {
        let recent_trades = KIRecentTrades {
            params: IndexMap::new()
        };
        recent_trades.for_pair(pair)
    }

    pub fn since(mut self, id: String) -> Self{
        self.params.insert(String::from("since"), id);
        self
    }
}

impl Input for KIRecentTrades {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Trades") },
           params: Some(self.params)
       }
    }
}

impl Pair for KIRecentTrades {
    fn get_list(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

pub struct KISpreadData {
    pub params: IndexMap<String, String>,
}

impl KISpreadData {
    pub fn build(pair: KAssetPair) -> Self {
        let spread = KISpreadData {
            params: IndexMap::new()
        };
        spread.for_pair(pair)
    }

    pub fn since(mut self, id: String) -> Self {
        self.params.insert(String::from("since"), id);
        self
    }
}

impl Input for KISpreadData {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Spread") },
           params: Some(self.params)
       }
    }
}

impl Pair for KISpreadData {
    fn get_list(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Time {
    /// as unix timestamp
    pub unixtime: i64,
    /// as RFC 1123 time format
    pub rfc1123: String,
}

/// A currency asset
#[derive(Deserialize, Serialize, Debug)]
pub struct KOAsset {
    /// asset class
    pub aclass: String,
    /// alternate name
    pub altname: String,
    /// scaling decimal places for record keeping
    pub decimals: u32,
    /// scaling decimal places for output display
    pub display_decimals: u32,
}

/// Ticker info
#[derive(Deserialize, Serialize, Debug)]
pub struct Tick {
    /// ask array(<price>, <whole lot volume>, <lot volume>)
    pub a: Vec<String>,
    /// bid array(<price>, <whole lot volume>, <lot volume>)
    pub b: Vec<String>,
    /// last trade closed array(<price>, <lot volume>)
    pub c: Vec<String>,
    /// volume array(<today>, <last 24 hours>)
    pub v: Vec<String>,
    /// volume weighted average price array(<today>, <last 24 hours>)
    pub p: Vec<String>,
    /// number of trades array(<today>, <last 24 hours>)
    pub t: Vec<u32>,
    /// low array(<today>, <last 24 hours>)
    pub l: Vec<String>,
    /// high array(<today>, <last 24 hours>)
    pub h: Vec<String>,
    /// today's opening price
    pub o: String,
}

/// Tradable asset pairs
#[derive(Deserialize, Serialize, Debug)]
pub struct AssetPair {
    /// alternate pair name
    pub altname: String,
    /// asset class of base component
    pub aclass_base: String,
    /// asset id of base component
    pub base: String,
    /// asset class of quote component
    pub aclass_quote: String,
    /// asset id of quote component
    pub quote: String,
    /// volume lot size
    pub lot: String,
    /// scaling decimal places for pair
    pub pair_decimals: u32,
    /// scaling decimal places for volume
    pub lot_decimals: u32,
    /// amount to multiply lot volume by to get currency volume
    pub lot_multiplier: u32,
    /// array of leverage amounts available when buying
    pub leverage_buy: Vec<u32>,
    /// array of leverage amounts available when selling
    pub leverage_sell: Vec<u32>,
    /// fee schedule array in [volume, percent fee] tuples
    pub fees: Vec<(u64, f64)>,
    /// maker fee schedule array in [volume, percent fee] tuples (if on maker/taker)
    pub fees_maker: Option<Vec<(u64, f64)>>,
    /// volume discount currency
    pub fee_volume_currency: String,
    /// margin call level
    pub margin_call: u32,
    /// stop-out/liquidation margin level
    pub margin_stop: u32,
}

/// Open High Low Close data
pub type OHLC = HashMap<String, serde_json::Value>;

#[derive(Deserialize, Serialize, Debug)]
pub struct DepthPairTuple(String, String, i64);

#[derive(Deserialize, Serialize, Debug)]
pub struct DepthPair {
    pub asks: Vec<DepthPairTuple>,
    pub bids: Vec<DepthPairTuple>,
}

pub type Depth = HashMap<String, DepthPair>;

#[derive(Deserialize, Serialize, Debug)]
pub struct KrakenResult<T> {
    pub error: Vec<String>,
    pub result: Option<T>,
}
