use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use indexmap::map::IndexMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{
    EndpointInfo, KAsset,
    KAssetPair, KrakenInput,
    LedgerType, MethodType,
    OrderCloseTime, OrderType,
    OrderFlags, TradeHistoryType, 
    TradeType
};

// Traits
use super::{
    InputList, InputListItem, Input, 
    IntoInputList, MutateInput, 
    UpdateInput
};

/// Request builder for the Get Open Positions endpoint 
pub struct KIOpenPositions {
    params: IndexMap<String, String>,
}

impl KIOpenPositions {
    pub fn build(txid: String) -> Self {
        let open_positions = KIOpenPositions {
            params: IndexMap::new()
        };
        open_positions.with_item(txid)
    }

    pub fn build_with_list<T>(txids: T) -> Self
        where T: IntoIterator<Item = String>
    {
        let open_positions = KIOpenPositions {
            params: IndexMap::new()
        };
        open_positions.with_item_list(txids)
    }

    pub fn update_transaction_list<T>(self, txids: T) -> Self
        where T: IntoIterator<Item = String>
    {
        self.update_input("txid", String::from("")).with_item_list(txids)
    }

    pub fn do_cals(self, docalcs: bool) -> Self {
        self.update_input("docalcs", docalcs.to_string())
    }

    // FIXME: Currently there is no way to disable the consolidation data point
    // In general, it's probably better to create new builders if you need to remove fields from
    // a query. We could allow all input methods to deal with options and then remove input fields
    // if a. the field already exists and b. None is passed in by the user, but I feel this would
    // muddy the interface unnecessarily
    pub fn consolidate(self) -> Self {
        self.update_input("consolidation", String::from("market"))
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIOpenPositions {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("OpenPositions") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("OpenPositions") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KIOpenPositions {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIOpenPositions {}

impl IntoInputList for KIOpenPositions {
    fn list_name(&self) -> String {
        String::from("txid")
    }
}

impl InputListItem for KIOpenPositions {
    type ListItem = String;
}

impl InputList for KIOpenPositions {}

/// Open position info data 
#[derive(Deserialize, Serialize, Debug)]
pub struct KOPositionInfo {
    /// Order responsible for execution of trade
    pub ordertxid: String,
    pub pair: String,
    pub time: f64,
    #[serde(rename = "type")]
    pub tradetype: String,
    pub ordertype: String,
    pub cost: String,
    pub fee: String,
    pub vol: String,
    pub vol_closed: String,
    pub margin: Option<String>,
    pub value: Option<String>,
    pub net: Option<String>,
    pub misc: String,
    pub oflags: Option<String>,
}

/// Response from the Get Open Positions endpoint 
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOpenPositions {
    /// Map with the position's transaction ID as the key and the open position info as the value
    #[serde(flatten)]
    pub positions: HashMap<String, KOPositionInfo>
}

