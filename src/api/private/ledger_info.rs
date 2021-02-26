use indexmap::map::IndexMap;

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{
    EndpointInfo, KAsset,
    KrakenInput,
    LedgerType, MethodType,
};

// Traits
use super::{
    InputList, InputListItem, Input, 
    IntoInputList, MutateInput, 
    UpdateInput
};

pub use super::KOLedgerInfo;
pub use super::KOLedgers;

/// Request builder for the Get Ledgers Info endpoint 
pub struct KILedgerInfo {
    params: IndexMap<String, String>,
}

impl KILedgerInfo {
    pub fn build() -> Self {
        KILedgerInfo {
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

    pub fn with_trade_type(self, ledgertype: LedgerType) -> Self {
        self.update_input("type", ledgertype.to_string())
    }

    pub fn from_timestamp(self, timestamp: u64) -> Self {
        self.update_input("start", timestamp.to_string())
    }

    pub fn to_timestamp(self, timestamp: u64) -> Self {
        self.update_input("end", timestamp.to_string())
    }

    pub fn from_txid(self, txid: String) -> Self {
        self.update_input("start", txid)
    }

    pub fn to_txid(self, txid: String) -> Self {
        self.update_input("end", txid)
    }

    pub fn with_offset(self, offset: u64) -> Self {
        self.update_input("ofs", offset.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KILedgerInfo {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("Ledgers") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("Ledgers") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KILedgerInfo {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KILedgerInfo {}

impl IntoInputList for KILedgerInfo {
    fn list_name(&self) -> String {
        String::from("asset")
    }
}

impl InputListItem for KILedgerInfo {
    type ListItem = KAsset;
}

impl InputList for KILedgerInfo {}


