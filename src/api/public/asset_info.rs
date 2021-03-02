use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    EndpointInfo, Input, InputList, InputListItem, IntoInputList, KAsset, KrakenInput, MethodType,
    MutateInput, UpdateInput,
};

/// Request builder for the Get Asset Info endpoint
pub struct KIAssetInfo {
    pub params: IndexMap<String, String>,
}

impl KIAssetInfo {
    pub fn build() -> Self {
        KIAssetInfo {
            params: IndexMap::new(),
        }
    }

    pub fn clear_asset_list(self) -> Self {
        self.update_input("asset", String::from(""))
    }

    pub fn with_asset(self, asset: KAsset) -> Self {
        self.with_item(asset)
    }

    pub fn with_asset_list<T>(self, assets: T) -> Self
    where
        T: IntoIterator<Item = KAsset>,
    {
        self.with_item_list(assets)
    }
}

impl Input for KIAssetInfo {
    fn finish(self) -> KrakenInput {
        KrakenInput {
            info: EndpointInfo {
                methodtype: MethodType::Public,
                endpoint: String::from("Assets"),
            },
            params: Some(self.params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Public,
                    endpoint: String::from("Assets"),
                },
                params: Some(self.params.clone()),
            },
            self,
        )
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

/// Asset info data
#[derive(Deserialize, Serialize, Debug)]
pub struct KOAsset {
    /// alternate name
    pub altname: String,
    /// asset class
    pub aclass: String,
    /// scaling decimal places for record keeping
    pub decimals: u32,
    /// scaling decimal places for output display
    pub display_decimals: u32,
}

/// Response from the Get Asset Info endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOAssetInfo {
    /// Map with the asset as the key and the asset's data as the value
    #[serde(flatten)]
    pub asset: HashMap<String, KOAsset>,
}
