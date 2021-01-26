use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use indexmap::map::IndexMap;

use super::{
    AssetPairInfo, InputList, InputListItem,
    EndpointInfo, Input, KAsset, 
    KAssetPair, KrakenInput, 
    privatemod::{IntoInputList, MutateInput}, OHLCInterval, 
    MethodType, UpdateInput
};

pub struct KIServerTime();

impl KIServerTime {
    pub fn build() -> KrakenInput {
        let time = KIServerTime();
        time.finish()
    }

    pub fn build_clone() -> (KrakenInput, Self) {
        let time = KIServerTime();
        time.finish_clone()
    }
}

impl Input for KIServerTime {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Time") },
           params: None 
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Time") },
           params: None 
        },
        self)
    }
}

pub struct KISystemStatus();

impl KISystemStatus {
    pub fn build() -> KrakenInput {
        let status = KISystemStatus();
        status.finish()
    }

    pub fn build_clone() -> (KrakenInput, Self) {
        let status = KISystemStatus();
        status.finish_clone()
    }
}

impl Input for KISystemStatus {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("SystemStatus") },
           params: None
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("SystemStatus") },
           params: None
       },
       self)
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

    pub fn clear_asset_list(self) -> Self {
        self.update_input("asset", String::from(""))
    }

    pub fn with_asset(self, asset: KAsset) -> Self {
        self.with_item(asset)
    }

    pub fn with_asset_list<T>(self, assets: T) -> Self
        where T: IntoIterator<Item = KAsset>
    {
        self.with_item_list(assets)
    }
}

impl Input for KIAssetInfo {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Assets") },
           params: Some(self.params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Assets") },
           params: Some(self.params.clone())
       },
       self)
    }
}

impl MutateInput for KIAssetInfo {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl IntoInputList for KIAssetInfo {
    fn list_name(&self) -> String {
        String::from("asset")
    }
}

impl InputListItem for KIAssetInfo {
    type ListItem = KAsset;
}

impl UpdateInput for KIAssetInfo {}

impl InputList for KIAssetInfo {}

pub struct KIAssetPairs {
    pub params: IndexMap<String, String>,
}

impl KIAssetPairs {
    pub fn build() -> Self {
        KIAssetPairs {
            params: IndexMap::new()
        }
    }

    pub fn clear_pair_list(self) -> Self {
        self.update_input("pair", String::from(""))
    }

    pub fn with_asset_pair(self, pair: KAssetPair) -> Self {
        self.with_item(pair)
    }

    pub fn with_asset_pair_list<T>(self, pairs: T) -> Self
        where T: IntoIterator<Item = KAssetPair>
    {
        self.with_item_list(pairs)
    }

    pub fn info (self, info: AssetPairInfo) -> Self {
        self.update_input("info", info.to_string())
    }
}

impl Input for KIAssetPairs {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("AssetPairs") },
           params: Some(self.params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("AssetPairs") },
           params: Some(self.params.clone())
       },
       self)
    }
}

impl MutateInput for KIAssetPairs {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIAssetPairs {}

impl IntoInputList for KIAssetPairs {
    fn list_name(&self) -> String {
        String::from("pair")
    }
}

impl InputListItem for KIAssetPairs {
    type ListItem = KAssetPair;
}

impl InputList for KIAssetPairs {}

pub struct KITicker {
    pub params: IndexMap<String, String>,
}

impl KITicker {
    pub fn build(pair: KAssetPair) -> Self {
        let ticker = KITicker {
            params: IndexMap::new()
        };
        ticker.with_item(pair)
    }

    pub fn update_pair_list<T>(self, pairs: T) -> Self
        where T: IntoIterator<Item = KAssetPair>
    {
        self.update_input("pair", String::from("")).with_item_list(pairs)
    }

    pub fn build_with_list<T>(pairs: T) -> Self 
        where T: IntoIterator<Item = KAssetPair>,
    {
        let ticker = KITicker {
            params: IndexMap::new()
        };
        ticker.with_item_list(pairs)
    }
}

impl Input for KITicker {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Ticker") },
           params: Some(self.params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Ticker") },
           params: Some(self.params.clone())
       },
       self)
    }
}

impl MutateInput for KITicker {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl IntoInputList for KITicker {
    fn list_name(&self) -> String {
        String::from("pair")
    }
}

impl InputListItem for KITicker {
    type ListItem = KAssetPair;
}

impl UpdateInput for KITicker {}

impl InputList for KITicker {}

pub struct KIOHLC {
    pub params: IndexMap<String, String>,
}

impl KIOHLC {
    pub fn build(pair: KAssetPair) -> Self {
        let ohlc = KIOHLC {
            params: IndexMap::new()
        };
        ohlc.update_pair(pair)
    }

    pub fn update_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    pub fn with_interval (self, interval: OHLCInterval) -> Self {
        self.update_input("interval", interval.to_string())
    }

    pub fn since(self, id: String) -> Self {
        self.update_input("since", id)
    }
}

impl Input for KIOHLC {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("OHLC") },
           params: Some(self.params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("OHLC") },
           params: Some(self.params.clone())
       },
       self)
    }
}

impl MutateInput for KIOHLC {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIOHLC {}

pub struct KIOrderBook {
    pub params: IndexMap<String, String>,
}

impl KIOrderBook {
    pub fn build(pair: KAssetPair) -> Self {
        let order_book = KIOrderBook {
            params: IndexMap::new()
        };
        order_book.update_pair(pair)
    }

    pub fn update_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    pub fn with_max (self, max: i64) -> Self {
        self.update_input("count", max.to_string())
    }
}

impl Input for KIOrderBook {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Depth") },
           params: Some(self.params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Depth") },
           params: Some(self.params.clone())
       },
       self)
    }
}

impl MutateInput for KIOrderBook {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIOrderBook {}

pub struct KIRecentTrades {
    pub params: IndexMap<String, String>,
}

impl KIRecentTrades {
    pub fn build(pair: KAssetPair) -> Self {
        let recent_trades = KIRecentTrades {
            params: IndexMap::new()
        };
        recent_trades.update_pair(pair)
    }

    pub fn update_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    pub fn since(self, id: String) -> Self{
        self.update_input("since", id)
    }
}

impl Input for KIRecentTrades {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Trades") },
           params: Some(self.params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Trades") },
           params: Some(self.params.clone())
       },
       self)
    }
}

impl MutateInput for KIRecentTrades {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIRecentTrades {}

pub struct KISpreadData {
    pub params: IndexMap<String, String>,
}

impl KISpreadData {
    pub fn build(pair: KAssetPair) -> Self {
        let spread = KISpreadData {
            params: IndexMap::new()
        };
        spread.update_pair(pair)
    }

    pub fn update_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    pub fn since(self, id: String) -> Self {
        self.update_input("since", id)
    }
}

impl Input for KISpreadData {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Spread") },
           params: Some(self.params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Public, endpoint: String::from("Spread") },
           params: Some(self.params.clone())
       },
       self)
    }
}

impl MutateInput for KISpreadData {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KISpreadData {}

#[derive(Deserialize, Serialize, Debug)]
pub struct KOServerTime {
    /// as unix timestamp
    pub unixtime: u64,
    /// as RFC 1123 time format
    pub rfc1123: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KOSystemStatus {
    status: String,
    timestamp: String,
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

/// Tradable asset pairs
#[derive(Deserialize, Serialize, Debug)]
pub struct KOAssetPair {
    /// alternate pair name
    pub altname: String,
    /// websocket pair name (if available)
    pub wsname: String,
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
    /// minimum order volume for pair
    pub ordermin: String,
}

/// Ticker info
#[derive(Deserialize, Serialize, Debug)]
pub struct KOTick {
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


/// Open High Low Close data
pub type KOOHLC = HashMap<String, serde_json::Value>;

#[derive(Deserialize, Serialize, Debug)]
pub struct KODepthPairTuple(String, String, i64);

#[derive(Deserialize, Serialize, Debug)]
pub struct KODepthPair {
    pub asks: Vec<KODepthPairTuple>,
    pub bids: Vec<KODepthPairTuple>,
}

pub type Depth = HashMap<String, KODepthPair>;

