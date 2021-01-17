use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::iter::Iterator;
use indexmap::map::IndexMap;

use crate::auth::KrakenAuth;
use crate::client::KrakenClient;
use crate::api::{Input, KAsset, KAssetPair, KrakenInput, MethodType, EndpointInfo};

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
           info: EndpointInfo { methodtype: MethodType::PUBLIC, endpoint: String::from("Time") },
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
           info: EndpointInfo { methodtype: MethodType::PUBLIC, endpoint: String::from("SystemStatus") },
           params: None
       }
    }
}

pub struct KIAssetInfo {
    pub params: IndexMap<String, String>,
}

impl KIAssetInfo {
    pub fn build() -> KIAssetInfo {
        KIAssetInfo {
            params: IndexMap::new()
        }
    }

    pub fn asset(mut self, asset: KAsset) -> KIAssetInfo {
        self.format(asset);
        self
    }

    // Fun stuff. If there exists a list of assets (previously called asset()), then iterate
    // over the list and comma-delimit the items. If no list exists before calling asset_list(),
    // first consume the first item and then recursivly consume the rest. Note the recursion consumes self 
    // and is equivalent to chaining calls to pair()
    pub fn asset_list<T>(mut self, pairs: T) -> KIAssetInfo 
        where T: IntoIterator<Item = KAsset>,
    {
        match self.params.contains_key("asset") {
            true => {
                pairs.into_iter().for_each(|pair| self.format(pair));
                self
            },
            false => {
                let mut iter = pairs.into_iter();
                self.params.insert(String::from("asset"), iter.next().unwrap().to_string());
                self.asset_list(iter)
            }
        }
    }

    fn format(&mut self, asset: KAsset) {
        // Either create a new asset or chain multiple assets into a comma separated list
        match self.params.get_mut("asset") {
            Some(list) => {
                // FIXME: Find a way to avoid extra allocation
                *list = format!("{},{}", list, asset.to_string());
            },
            None => {
                self.params.insert("asset".to_string(), asset.to_string());
            },
        }
    }
}

impl Input for KIAssetInfo {
    fn finish_input(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::PUBLIC, endpoint: String::from("Assets") },
           params: Some(self.params)
       }
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
pub struct Asset {
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
