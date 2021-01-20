use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use indexmap::map::IndexMap;

use super::super::auth::KrakenAuth;
// Structs/Enums
use super::{
    EndpointInfo, KAsset,
    KAssetPair, KrakenInput,
    LedgerType, MethodType,
    OrderCloseTime, TradeHistoryType
};

// Traits
use super::{
    InputList, InputListItem, Input, 
    privatemod::IntoInputList, privatemod::MutateInput, 
    UpdateInput
};

pub struct KIAccountBalance {
    pub params: IndexMap<String, String>,
}

impl KIAccountBalance {
    pub fn build() -> KrakenInput {
        let account_balance = KIAccountBalance {
            params: IndexMap::new()
        };
        account_balance.finish()
    }

    pub fn build_clone() -> (KrakenInput, Self) {
        let account_balance = KIAccountBalance {
            params: IndexMap::new()
        };
        account_balance.finish_clone()
    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIAccountBalance {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("Balance") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
       info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("Balance") },
       params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KIAccountBalance {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIAccountBalance {}

pub struct KITradeBalance {
    pub params: IndexMap<String, String>,
}

impl KITradeBalance {
    pub fn build() -> KITradeBalance {
        KITradeBalance {
            params: IndexMap::new()
        }
    }

    // FIXME: All instances of with_nonce need to handle updating the nonce if the key-value
    // already exists
    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
    }
}

impl MutateInput for KITradeBalance {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KITradeBalance {}

impl IntoInputList for KITradeBalance {
    fn list_name(&self) -> String {
        String::from("asset")
    }
}

impl InputListItem for KITradeBalance {
    type ListItem = KAsset;
}

impl Input for KITradeBalance {
    fn finish(self) -> KrakenInput {
        KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("TradeBalance") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("TradeBalance") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

pub struct KIOpenOrders {
    pub params: IndexMap<String, String>,
}

impl KIOpenOrders {
    pub fn build() -> Self {
        KIOpenOrders {
            params: IndexMap::new()
        }
    }

    pub fn with_trade_info(mut self) -> Self {
        self.params.insert(String::from("trades"), String::from("true"));
        self
    }

    pub fn with_userref (mut self, userref: &str) -> Self {
        match self.params.get_mut("userref") {
            Some(uref) => {
                *uref = userref.to_string();
                self
            }
            None => {
                self.params.insert(String::from("userref"), userref.to_string());
                self
            }
        }
    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIOpenOrders {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("OpenOrders") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("OpenOrders") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KIOpenOrders {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIOpenOrders {}

pub struct KIClosedOrders {
    pub params: IndexMap<String, String>,
}

impl KIClosedOrders {
    pub fn build() -> Self {
        KIClosedOrders {
            params: IndexMap::new()
        }
    }

    pub fn with_trade_info(mut self) -> Self {
        self.params.insert(String::from("trades"), String::from("true"));
        self
    }

    pub fn with_userref (mut self, userref: &str) -> Self {
        match self.params.get_mut("userref") {
            Some(uref) => {
                *uref = userref.to_string();
                self
            }
            None => {
                self.params.insert(String::from("userref"), userref.to_string());
                self
            }
        }
    }

    pub fn from_timestamp(mut self, timestamp: u64) -> Self {
        match self.params.get_mut("start") {
            Some(time) => {
                *time = timestamp.to_string();
                self
            }
            None => {
                self.params.insert(String::from("start"), timestamp.to_string());
                self
            }
        }
    }

    pub fn to_timestamp(mut self, timestamp: u64) -> Self {
        match self.params.get_mut("end") {
            Some(time) => {
                *time = timestamp.to_string();
                self
            }
            None => {
                self.params.insert(String::from("end"), timestamp.to_string());
                self
            }
        }
    }

    pub fn from_txid(mut self, txid: u64) -> Self {
        match self.params.get_mut("start") {
            Some(time) => {
                *time = txid.to_string();
                self
            }
            None => {
                self.params.insert(String::from("start"), txid.to_string());
                self
            }
        }
    }

    pub fn to_txid(mut self, txid: u64) -> Self {
        match self.params.get_mut("end") {
            Some(time) => {
                *time = txid.to_string();
                self
            }
            None => {
                self.params.insert(String::from("end"), txid.to_string());
                self
            }
        }
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        match self.params.get_mut("ofs") {
            Some(ofs) => {
                *ofs = offset.to_string();
                self
            }
            None => {
                self.params.insert(String::from("ofs"), offset.to_string());
                self
            }
        }
    }

    pub fn with_closetime(mut self, closetime: OrderCloseTime) -> Self {
        match self.params.get_mut("closetime") {
            Some(ct) => {
                *ct = closetime.to_string();
                self
            }
            None => {
                self.params.insert(String::from("closetime"), closetime.to_string());
                self
            }
        }

    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIClosedOrders {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("ClosedOrders") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("ClosedOrders") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KIClosedOrders {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIClosedOrders {}

pub struct KIOrderInfo {
    pub params: IndexMap<String, String>,
}

impl KIOrderInfo {
    pub fn build(txid: u64) -> Self {
        let order_info = KIOrderInfo {
            params: IndexMap::new()
        };
        order_info.for_item(txid)
    }

    pub fn build_with_list<T>(txids: T) -> Self
        where T: IntoIterator<Item = u64>
    {
        let order_info = KIOrderInfo {
            params: IndexMap::new()
        };
        order_info.for_item_list(txids)
    }

    pub fn with_trade_info(mut self) -> Self {
        self.params.insert(String::from("trades"), String::from("true"));
        self
    }

    pub fn with_userref (mut self, userref: &str) -> Self {
        match self.params.get_mut("userref") {
            Some(uref) => {
                *uref = userref.to_string();
                self
            }
            None => {
                self.params.insert(String::from("userref"), userref.to_string());
                self
            }
        }
    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIOrderInfo {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("QueryOrders") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("QueryOrders") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KIOrderInfo {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIOrderInfo {}

impl IntoInputList for KIOrderInfo {
    fn list_name(&self) -> String {
        String::from("txid")
    }
}

impl InputListItem for KIOrderInfo {
    type ListItem = u64;
}

impl InputList for KIOrderInfo {}

pub struct KITradeHistory {
    pub params: IndexMap<String, String>,
}

impl KITradeHistory {
    pub fn build() -> Self {
        KITradeHistory {
            params: IndexMap::new()
        }
    }

    pub fn with_trade_type(mut self, tradetype: TradeHistoryType) -> Self {
        match self.params.get_mut("type") {
            Some(ty) => {
                *ty = tradetype.to_string();
                self
            }
            None => {
                self.params.insert(String::from("type"), tradetype.to_string());
                self
            }
        }
    }

    pub fn with_trade_info(mut self) -> Self {
        self.params.insert(String::from("trades"), String::from("true"));
        self
    }

    pub fn from_timestamp(mut self, timestamp: u64) -> Self {
        match self.params.get_mut("start") {
            Some(time) => {
                *time = timestamp.to_string();
                self
            }
            None => {
                self.params.insert(String::from("start"), timestamp.to_string());
                self
            }
        }
    }

    pub fn to_timestamp(mut self, timestamp: u64) -> Self {
        match self.params.get_mut("end") {
            Some(time) => {
                *time = timestamp.to_string();
                self
            }
            None => {
                self.params.insert(String::from("end"), timestamp.to_string());
                self
            }
        }
    }

    pub fn from_txid(mut self, txid: u64) -> Self {
        match self.params.get_mut("start") {
            Some(time) => {
                *time = txid.to_string();
                self
            }
            None => {
                self.params.insert(String::from("start"), txid.to_string());
                self
            }
        }
    }

    pub fn to_txid(mut self, txid: u64) -> Self {
        match self.params.get_mut("end") {
            Some(time) => {
                *time = txid.to_string();
                self
            }
            None => {
                self.params.insert(String::from("end"), txid.to_string());
                self
            }
        }
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        match self.params.get_mut("ofs") {
            Some(ofs) => {
                *ofs = offset.to_string();
                self
            }
            None => {
                self.params.insert(String::from("ofs"), offset.to_string());
                self
            }
        }
    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
    }
}

impl Input for KITradeHistory {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("TradesHistory") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("TradesHistory") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KITradeHistory {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KITradeHistory {}

pub struct KITradesInfo {
    pub params: IndexMap<String, String>,
}

impl KITradesInfo {
    pub fn build(txid: u64) -> Self {
        let trades_info = KITradesInfo {
            params: IndexMap::new()
        };
        trades_info.for_item(txid)
    }

    pub fn build_with_list<T>(txids: T) -> Self
        where T: IntoIterator<Item = u64>
    {
        let trades_info = KITradesInfo {
            params: IndexMap::new()
        };
        trades_info.for_item_list(txids)
    }

    pub fn with_trade_info(mut self) -> Self {
        self.params.insert(String::from("trades"), String::from("true"));
        self
    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
    }
}

impl Input for KITradesInfo {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("QueryTrades") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("QueryTrades") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KITradesInfo {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KITradesInfo {}

impl IntoInputList for KITradesInfo {
    fn list_name(&self) -> String {
        String::from("txid")
    }
}

impl InputListItem for KITradesInfo {
    type ListItem = u64;
}

impl InputList for KITradesInfo {}

pub struct KIOpenPositions {
    pub params: IndexMap<String, String>,
}

impl KIOpenPositions {
    pub fn build(txid: u64) -> Self {
        let open_positions = KIOpenPositions {
            params: IndexMap::new()
        };
        open_positions.for_item(txid)
    }

    pub fn build_with_list<T>(txids: T) -> Self
        where T: IntoIterator<Item = u64>
    {
        let open_positions = KIOpenPositions {
            params: IndexMap::new()
        };
        open_positions.for_item_list(txids)
    }

    pub fn do_cals(mut self) -> Self {
        self.params.insert(String::from("docalcs"), String::from("true"));
        self
    }

    pub fn consolidate(mut self) -> Self {
        self.params.insert(String::from("consolidation"), String::from("market"));
        self
    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
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
    type ListItem = u64;
}

impl InputList for KIOpenPositions {}

pub struct KILedgerInfo {
    pub params: IndexMap<String, String>,
}

impl KILedgerInfo {
    pub fn build() -> Self {
        KILedgerInfo {
            params: IndexMap::new()
        }
    }

    pub fn with_trade_type(mut self, ledgertype: LedgerType) -> Self {
        match self.params.get_mut("type") {
            Some(ty) => {
                *ty = ledgertype.to_string();
                self
            }
            None => {
                self.params.insert(String::from("type"), ledgertype.to_string());
                self
            }
        }
    }

    pub fn from_timestamp(mut self, timestamp: u64) -> Self {
        match self.params.get_mut("start") {
            Some(time) => {
                *time = timestamp.to_string();
                self
            }
            None => {
                self.params.insert(String::from("start"), timestamp.to_string());
                self
            }
        }
    }

    pub fn to_timestamp(mut self, timestamp: u64) -> Self {
        match self.params.get_mut("end") {
            Some(time) => {
                *time = timestamp.to_string();
                self
            }
            None => {
                self.params.insert(String::from("end"), timestamp.to_string());
                self
            }
        }
    }

    pub fn from_txid(mut self, txid: u64) -> Self {
        match self.params.get_mut("start") {
            Some(time) => {
                *time = txid.to_string();
                self
            }
            None => {
                self.params.insert(String::from("start"), txid.to_string());
                self
            }
        }
    }

    pub fn to_txid(mut self, txid: u64) -> Self {
        match self.params.get_mut("end") {
            Some(time) => {
                *time = txid.to_string();
                self
            }
            None => {
                self.params.insert(String::from("end"), txid.to_string());
                self
            }
        }
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        match self.params.get_mut("ofs") {
            Some(ofs) => {
                *ofs = offset.to_string();
                self
            }
            None => {
                self.params.insert(String::from("ofs"), offset.to_string());
                self
            }
        }
    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
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

pub struct KIQueryLedgers {
    pub params: IndexMap<String, String>,
}

impl KIQueryLedgers {
    pub fn build(txid: u64) -> Self {
        let trades_info = KIQueryLedgers {
            params: IndexMap::new()
        };
        trades_info.for_item(txid)
    }

    pub fn build_with_list<T>(txids: T) -> Self
        where T: IntoIterator<Item = u64>
    {
        let trades_info = KIQueryLedgers {
            params: IndexMap::new()
        };
        trades_info.for_item_list(txids)
    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
    }
}

impl Input for KIQueryLedgers {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("QueryLedgers") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("QueryLedgers") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KIQueryLedgers {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIQueryLedgers {}

impl IntoInputList for KIQueryLedgers {
    fn list_name(&self) -> String {
        String::from("id")
    }
}

impl InputListItem for KIQueryLedgers {
    type ListItem = u64;
}

impl InputList for KIQueryLedgers {}

pub struct KITradeVolume {
    pub params: IndexMap<String, String>,
}

impl KITradeVolume {
    pub fn build() -> Self {
        KITradeVolume {
            params: IndexMap::new()
        }
    }

    pub fn with_fee_info(mut self) -> Self {
        self.params.insert(String::from("fee-info"), String::from("true"));
        self
    }

    fn with_nonce(self) -> Self {
        self.update_item("nonce", KrakenAuth::nonce())
    }
}

impl MutateInput for KITradeVolume {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KITradeVolume {}

impl IntoInputList for KITradeVolume {
    fn list_name(&self) -> String {
        String::from("pair")
    }
}

impl InputListItem for KITradeVolume {
    type ListItem = KAssetPair;
}

impl InputList for KITradeVolume {}

impl Input for KITradeVolume {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("TradeVolume") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("TradeVolume") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TradeBalance {
    /// equivalent balance (combined balance of all currencies)
    pub eb: String,
    /// trade balance (combined balance of all equity currencies)
    pub tb: String,
    /// margin amount of open positions
    pub m: String,
    /// unrealized net profit/loss of open positions
    pub n: String,
    /// cost basis of open positions
    pub c: String,
    /// current floating valuation of open positions
    pub v: String,
    /// equity = trade balance + unrealized net profit/loss
    pub e: String,
    /// free margin = equity - initial margin (maximum margin available to open new positions)
    pub mf: String,
    /// margin level = (equity / initial margin) * 100
    pub ml: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderDescription {
    pub leverage: String,
    pub order: String,
    pub ordertype: String,
    pub pair: String,
    pub price: String,
    pub price2: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// order pending book entry
    Pending,
    /// open order
    Open,
    /// closed order
    Closed,
    /// order canceled
    Canceled,
    /// order expired
    Expired,
}

/// General order info object
#[derive(Deserialize, Serialize, Debug)]
pub struct OrderInfo {
    /// unix timestamp of when order was closed
    pub closetm: Option<f64>,
    /// total cost (quote currency unless unless viqc set in oflags)
    pub cost: String,
    pub descr: OrderDescription,
    /// unix timestamp of order end time (or 0 if not set)
    pub expiretm: f64,
    /// total fee (quote currency)
    pub fee: String,
    /// comma delimited list of miscellaneous info:
    /// + stopped = triggered by stop price
    /// + touched = triggered by touch price
    /// + liquidated = liquidation
    /// + partial = partial fill
    pub misc: String,
    /// comma delimited list of order flags:
    /// + viqc = volume in quote currency
    /// + fcib = prefer fee in base currency (default if selling)
    /// + fciq = prefer fee in quote currency (default if buying)
    /// + nompp = no market price protection
    pub oflags: String,
    /// unix timestamp of when order was placed
    pub opentm: f64,
    /// average price (quote currency unless viqc set in oflags)
    pub price: String,
    /// stop price (quote currency, for trailing stops)
    pub stopprice: Option<String>,
    /// triggered limit price (quote currency, when limit based order type triggered)
    pub limitprice: Option<String>,
    /// additional info on status (if any)
    pub reason: Option<String>,
    /// Referral order transaction id that created this order
    pub refid: Option<String>,
    /// unix timestamp of order start time (or 0 if not set)
    pub starttm: f64,
    /// status of order:
    pub status: OrderStatus,
    /// user reference id
    pub userref: Option<String>,
    /// volume of order (base currency unless viqc set in oflags)
    pub vol: String,
    /// volume executed (base currency unless viqc set in oflags)
    pub vol_exec: String,
}

/// Open orders
#[derive(Deserialize, Serialize, Debug)]
pub struct OpenOrders {
    pub open: HashMap<String, OrderInfo>,
}

/// Closed order result
#[derive(Deserialize, Serialize, Debug)]
pub struct ClosedOrders {
    pub closed: HashMap<String, OrderInfo>,
    pub count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ClosedOrdersConfigCloseTime {
    Open,
    Close,
    Both,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClosedOrdersConfig {
    /// whether or not to include trades in output (optional.  default = false).
    pub trades: Option<bool>,
    /// restrict results to given user reference id (optional).
    pub userref: Option<String>,
    /// starting unix timestamp or order tx id of results (optional.  exclusive).
    pub start: Option<i64>,
    /// ending unix timestamp or order tx id of results (optional.  inclusive).
    pub end: Option<i64>,
    /// result offset.
    pub ofs: Option<u64>,
    /// which time to use (optional).
    pub closetime: Option<ClosedOrdersConfigCloseTime>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TradeType {
    Buy,
    Sell,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum OrderType {
    Market,
    /// (price = limit price)
    Limit,
    /// (price = stop loss price)
    StopLoss,
    /// (price = take profit price)
    TakeProfit,
    /// (price = stop loss price, price2 = take profit price)
    StopLossProfit,
    /// (price = stop loss price, price2 = take profit price)
    StopLossProfitLimit,
    /// (price = stop loss trigger price, price2 = triggered limit price)
    StopLossLimit,
    /// (price = take profit trigger price, price2 = triggered limit price)
    TakeProfitLimit,
    /// (price = trailing stop offset)
    TrailingStop,
    /// (price = trailing stop offset, price2 = triggered limit offset)
    TrailingStopLimit,
    /// (price = stop loss price, price2 = limit price)
    StopLossAndLimit,
    SettlePosition,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewOrder {
    /// asset pair
    pub pair: String,
    /// type of order (buy/sell)
    pub kind: TradeType,
    pub order_type: OrderType,
    /// price (optional.  dependent upon ordertype)
    pub price: Option<String>,
    /// secondary price (optional.  dependent upon ordertype)
    pub price2: Option<String>,
    /// order volume in lots
    pub volume: String,
    /// amount of leverage desired (optional.  default = none)
    pub leverage: Option<String>,
    /// comma delimited list of order flags (optional):
    ///   + viqc = volume in quote currency (not available for leveraged orders)
    ///   + fcib = prefer fee in base currency
    ///   + fciq = prefer fee in quote currency
    ///   + nompp = no market price protection
    ///   + post = post only order (available when ordertype = limit)
    pub oflags: Option<String>,
    /// scheduled start time (optional):
    ///   + 0 = now (default)
    ///   + +<n> = schedule start time <n> seconds from now
    ///   + <n> = unix timestamp of start time
    pub starttm: Option<i64>,
    /// expiration time (optional):
    ///   + 0 = no expiration (default)
    ///   + +<n> = expire <n> seconds from now
    ///   + <n> = unix timestamp of expiration time
    pub expiretm: Option<i64>,
    /// user reference id.  32-bit signed number.  (optional)
    pub userref: Option<String>,
    /// validate inputs only.  do not submit order (optional)
    pub validate: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CanceldOrders {
    /// number of orders canceled
    count: u32,
    /// if set, order(s) is/are pending cancellation
    pending: u32,
}

