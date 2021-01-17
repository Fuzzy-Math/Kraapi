use std::fmt;
use indexmap::map::IndexMap;
use std::fmt::{Debug,Display};

pub mod public;
pub mod private;

// TODO: Query AssetInfo endpoint and write script to fill out the
// enum and trait impl
pub enum KAsset {
    EUR,
    USD,
    XBT,
    XRP,
}

impl std::fmt::Display for KAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KAsset::EUR => write!(f, "{}", "ZEUR"),
            KAsset::USD => write!(f, "{}", "ZUSD"),
            KAsset::XBT => write!(f, "{}", "XXBT"),
            KAsset::XRP => write!(f, "{}", "XXRP"),
        }
    }
}

impl Debug for KAsset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string().get(1..).unwrap())
    }
}

pub struct KAssetPair(KAsset, KAsset);

impl fmt::Display for KAssetPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0.to_string(), self.1.to_string())
    }
}

pub(crate) enum MethodType {
    PRIVATE,
    PUBLIC,
}

impl fmt::Display for MethodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MethodType::PRIVATE => write!(f, "{}", "private"),
            MethodType::PUBLIC => write!(f, "{}", "public"),
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

