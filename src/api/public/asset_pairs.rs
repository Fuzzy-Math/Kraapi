use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    AssetPairInfo, EndpointInfo, Input, InputList, InputListItem, IntoInputList, KAssetPair,
    KrakenInput, MethodType, MutateInput, Output, UpdateInput,
};

/// Request builder for the Get Tradable Asset Pairs endpoint
pub struct KIAssetPairs {
    params: IndexMap<String, String>,
}

impl KIAssetPairs {
    /// Constructor returning a [KrakenInput] builder for the get tradeable asset pairs endpoint.
    pub fn build() -> Self {
        KIAssetPairs {
            params: IndexMap::new(),
        }
    }

    /// An asset pair is not required for the get asset info endpoint. 
    /// This method clears a currently populated asset pair list.
    /// Useful for templating
    pub fn clear_pair_list(self) -> Self {
        self.update_input("pair", String::from(""))
    }

    /// Update the list of assets pairs to query info for 
    pub fn with_asset_pair(self, pair: KAssetPair) -> Self {
        self.with_item(pair)
    }

    /// Update the list of assets pairs to query info for 
    pub fn with_asset_pair_list<T>(self, pairs: T) -> Self
    where
        T: IntoIterator<Item = KAssetPair>,
    {
        self.with_item_list(pairs)
    }

    /// [Asset pair info][AssetPairInfo] to retrieve
    pub fn info(self, info: AssetPairInfo) -> Self {
        self.update_input("info", info.to_string())
    }
}

impl Input for KIAssetPairs {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Public,
                endpoint: String::from("AssetPairs"),
            },
            params: Some(self.params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Public,
                    endpoint: String::from("AssetPairs"),
                },
                params: Some(self.params.clone()),
            },
            self,
        )
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

/// Asset pair info data
#[derive(Deserialize, Serialize, Debug)]
pub struct KOAssetPair {
    /// asset class of base component
    pub aclass_base: String,
    /// asset class of quote component
    pub aclass_quote: String,
    /// alternate pair name
    pub altname: String,
    /// asset id of base component
    pub base: String,
    /// volume discount currency
    pub fee_volume_currency: String,
    /// fee schedule array in [volume, percent fee] tuples
    pub fees: Vec<(u64, f64)>,
    /// maker fee schedule array in [volume, percent fee] tuples (if on maker/taker)
    pub fees_maker: Option<Vec<(u64, f64)>>,
    /// array of leverage amounts available when buying
    pub leverage_buy: Vec<u32>,
    /// array of leverage amounts available when selling
    pub leverage_sell: Vec<u32>,
    /// volume lot size
    pub lot: String,
    /// scaling decimal places for volume
    pub lot_decimals: u32,
    /// amount to multiply lot volume by to get currency volume
    pub lot_multiplier: u32,
    /// margin call level
    pub margin_call: u32,
    /// stop-out/liquidation margin level
    pub margin_stop: u32,
    /// minimum order volume for pair
    pub ordermin: Option<String>,
    /// scaling decimal places for pair
    pub pair_decimals: u32,
    /// asset id of quote component
    pub quote: String,
    /// websocket pair name (if available)
    pub wsname: Option<String>,
}

/// Response from the Get Tradable Asset Pairs endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOAssetPairInfo {
    /// Map with the asset pair as the key and the pair's data as the value
    #[serde(flatten)]
    pub pair: HashMap<KAssetPair, KOAssetPair>,
}

impl Output for KOAssetPairInfo {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::asset::*;

    #[test]
    fn update_pairs_list() {
        let pairs = vec!{ KAssetPair(KAsset::XBT, KAsset::USD), 
            KAssetPair(KAsset::XBT, KAsset::CAD) };
        let asset_pairs1 = KIAssetPairs::build();
        let asset_pairs2 = KIAssetPairs::build();

        let asset_pairs1 = asset_pairs1.with_asset_pair(KAssetPair(KAsset::XBT, KAsset::USD));
        let asset_pairs1 = asset_pairs1.with_asset_pair(KAssetPair(KAsset::XBT, KAsset::CAD));

        let asset_pairs2 = asset_pairs2.with_asset_pair_list(pairs);

        assert_eq!(asset_pairs1.params.get("pair").unwrap(), 
                   asset_pairs2.params.get("pair").unwrap()
        );
    }
}
