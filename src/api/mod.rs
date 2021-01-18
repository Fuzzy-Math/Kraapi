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
    fn finish_input(self) -> KrakenInput;
}

pub trait Pair {
    fn get_list(&mut self) -> &mut IndexMap<String, String>;

    fn for_pair(mut self, pair: KAssetPair) -> Self 
        where Self: Sized
    {
        self.format(pair);
        self
    }

    fn format(&mut self, pair: KAssetPair) {
        match self.get_list().get_mut("pair") {
            Some(list) => {
                *list = format!("{},{}", list, pair.to_string());
            },
            None => {
                self.get_list().insert(String::from("pair"), pair.to_string());
            },
        }
    }
}

pub trait PairList : Pair { 
    // Fun stuff. If there exists a list of asset pairs (previously called for_pair()), then iterate
    // over the list and comma separate the items. If no list exists before calling for_pair_list(),
    // first consume the first item and then recursivly consume the rest. Note the recursion consumes self 
    // and is equivalent to chaining calls to for_pair()
    fn for_pair_list<U>(mut self, pairs: U) -> Self
        where U: IntoIterator<Item = KAssetPair>,
              Self: Sized
    {
        match self.get_list().contains_key("pair") {
            true => {
                pairs.into_iter().for_each(|pair| self.format(pair));
                self
            },
            false => {
                let mut iter = pairs.into_iter();
                self.get_list().insert(String::from("pair"), iter.next().unwrap().to_string());
                self.for_pair_list(iter)
            }
        }
    }
}

pub trait Asset {
    fn get_list(&mut self) -> &mut IndexMap<String, String>;

    fn for_asset(mut self, asset: KAsset) -> Self 
        where Self: Sized
    {
        self.format(asset);
        self
    }

    fn format(&mut self, asset: KAsset) {
        match self.get_list().get_mut("asset") {
            Some(list) => {
                *list = format!("{},{}", list, asset.to_string());
            },
            None => {
                self.get_list().insert(String::from("asset"), asset.to_string());
            },
        }
    }
}

pub trait AssetList : Asset { 
    // Fun stuff. If there exists a list of assets (previously called for_asset()), then iterate
    // over the list and comma separate the items. If no list exists before calling for_asset_list(),
    // first consume the first item and then recursivly consume the rest. Note the recursion consumes self 
    // and is equivalent to chaining calls to for_asset()
    fn for_asset_list<U>(mut self, assets: U) -> Self
        where U: IntoIterator<Item = KAsset>,
              Self: Sized
    {
        match self.get_list().contains_key("asset") {
            true => {
                assets.into_iter().for_each(|asset| self.format(asset));
                self
            },
            false => {
                let mut iter = assets.into_iter();
                self.get_list().insert(String::from("asset"), iter.next().unwrap().to_string());
                self.for_asset_list(iter)
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
