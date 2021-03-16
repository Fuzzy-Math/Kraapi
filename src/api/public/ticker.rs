use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    EndpointInfo, Input, InputList, InputListItem, IntoInputList, KAssetPair, KrakenInput,
    MethodType, MutateInput, Output, UpdateInput,
};

/// Request builder for the Get Ticker Information endpoint
pub struct KITicker {
    params: IndexMap<String, String>,
}

impl KITicker {
    /// Constructor returning a [KrakenInput] builder for the get ticker information endpoint.
    /// * `pair` is the asset pair to query info for
    pub fn build(pair: KAssetPair) -> Self {
        let ticker = KITicker {
            params: IndexMap::new(),
        };
        ticker.with_item(pair)
    }

    /// Constructor returning a [KrakenInput] builder for the get ticker information endpoint.
    /// * `pairs` is any iterable collection of asset pairs to query info for
    pub fn build_with_list<T>(pairs: T) -> Self
    where
        T: IntoIterator<Item = KAssetPair>,
    {
        let ticker = KITicker {
            params: IndexMap::new(),
        };
        ticker.with_item_list(pairs)
    }

    /// Update the list of asset pairs to query info for.
    /// Useful for templating
    pub fn update_pair_list<T>(self, pairs: T) -> Self
    where
        T: IntoIterator<Item = KAssetPair>,
    {
        self.update_input("pair", String::from(""))
            .with_item_list(pairs)
    }
}

impl Input for KITicker {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Public,
                endpoint: String::from("Ticker"),
            },
            params: Some(self.params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Public,
                    endpoint: String::from("Ticker"),
                },
                params: Some(self.params.clone()),
            },
            self,
        )
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

/// Ticker info data
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

/// Response from the Get Ticker Information endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOTicker {
    /// Map with the asset pair as the key and the pair's ticker data as the value
    #[serde(flatten)]
    pub pair: HashMap<KAssetPair, KOTick>,
}

impl Output for KOTicker {}
