//! Module for interacting with Kraken's private API endpoints
//! # Note
//! Each type prefixed with "KI" is a KrakenInput builder which will build requests for the given
//! endpoint.
//! Each type postfixed with "KO" is the output object that has been returned from Kraken's servers
//! and has been parsed into the given structure.
//! A valid api key and api secret will have to be used when creating a KrakenClient otherwise
//! requests sent to private endpoints will panic before being sent to Kraken

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use indexmap::map::IndexMap;

use super::super::auth::KrakenAuth;
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

/// Request builder for the Get Account Balance endpoint | See [KOAccountBalance]
pub struct KIAccountBalance {
    params: IndexMap<String, String>,
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
        self.update_input("nonce", KrakenAuth::nonce())
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

/// Request builder for the Get Trade Balance endpoint
pub struct KITradeBalance {
    params: IndexMap<String, String>,
}

impl KITradeBalance {
    pub fn build() -> KITradeBalance {
        KITradeBalance {
            params: IndexMap::new()
        }
    }

    pub fn with_asset(self, asset: KAsset) -> Self {
        self.update_input("asset", asset.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl MutateInput for KITradeBalance {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KITradeBalance {}

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

/// Request builder for the Get Open Orders endpoint
pub struct KIOpenOrders {
    params: IndexMap<String, String>,
}

impl KIOpenOrders {
    pub fn build() -> Self {
        KIOpenOrders {
            params: IndexMap::new()
        }
    }

    // FIXME: AFter testing, trades=false still causes trade data to be returned. So the entire key
    // value pair needs to be removed on false input
    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    pub fn with_userref (self, userref: u32) -> Self {
        self.update_input("userref", userref.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
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

/// Request builder for the Get Closed Orders endpoint
pub struct KIClosedOrders {
    params: IndexMap<String, String>,
}

impl KIClosedOrders {
    pub fn build() -> Self {
        KIClosedOrders {
            params: IndexMap::new()
        }
    }

    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    pub fn with_userref (self, userref: u32) -> Self {
        self.update_input("userref", userref.to_string())
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

    pub fn with_closetime(self, closetime: OrderCloseTime) -> Self {
        self.update_input("closetime", closetime.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
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

/// Request builder for the Query Orders Info endpoint
pub struct KIOrderInfo {
    params: IndexMap<String, String>,
}

impl KIOrderInfo {
    pub fn build(txid: String) -> Self {
        let order_info = KIOrderInfo {
            params: IndexMap::new()
        };
        order_info.with_item(txid)
    }

    pub fn build_with_list<T>(txids: T) -> Self
        where T: IntoIterator<Item = String>
    {
        let order_info = KIOrderInfo {
            params: IndexMap::new()
        };
        order_info.with_item_list(txids)
    }

    pub fn update_transaction_list<T>(self, txids: T) -> Self
        where T: IntoIterator<Item = String>
    {
        self.update_input("txid", String::from("")).with_item_list(txids)
    }

    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    pub fn with_userref (self, userref: u32) -> Self {
        self.update_input("userref", userref.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
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
    type ListItem = String;
}

impl InputList for KIOrderInfo {}

/// Request builder for the Get Trades History endpoint
pub struct KITradeHistory {
    params: IndexMap<String, String>,
}

impl KITradeHistory {
    pub fn build() -> Self {
        KITradeHistory {
            params: IndexMap::new()
        }
    }

    pub fn with_trade_type(self, tradetype: TradeHistoryType) -> Self {
        self.update_input("type", tradetype.to_string())
    }

    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
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

/// Request builder for the Query Trades Info endpoint
pub struct KITradesInfo {
    params: IndexMap<String, String>,
}

impl KITradesInfo {
    pub fn build(txid: String) -> Self {
        let trades_info = KITradesInfo {
            params: IndexMap::new()
        };
        trades_info.with_item(txid)
    }

    pub fn build_with_list<T>(txids: T) -> Self
        where T: IntoIterator<Item = String>
    {
        let trades_info = KITradesInfo {
            params: IndexMap::new()
        };
        trades_info.with_item_list(txids)
    }

    pub fn update_transaction_list<T>(self, txids: T) -> Self
        where T: IntoIterator<Item = String>
    {
        self.update_input("txid", String::from("")).with_item_list(txids)
    }

    pub fn with_trade_info(self, include_trades: bool) -> Self {
        if include_trades {
            self.update_input("trades", include_trades.to_string())
        } else {
            self.update_input("trades", String::from(""))
        }
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
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
    type ListItem = String;
}

impl InputList for KITradesInfo {}

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

/// Request builder for the Query Ledgers endpoint
pub struct KIQueryLedgers {
    params: IndexMap<String, String>,
}

impl KIQueryLedgers {
    pub fn build(ledgerid: String) -> Self {
        let ledgers = KIQueryLedgers {
            params: IndexMap::new()
        };
        ledgers.with_item(ledgerid)
    }

    pub fn build_with_list<T>(ledgerids: T) -> Self
        where T: IntoIterator<Item = String>
    {
        let ledgers = KIQueryLedgers {
            params: IndexMap::new()
        };
        ledgers.with_item_list(ledgerids)
    }

    pub fn update_transaction_list<T>(self, ledgerids: T) -> Self
        where T: IntoIterator<Item = String>
    {
        self.update_input("id", String::from("")).with_item_list(ledgerids)
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
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
    type ListItem = String;
}

impl InputList for KIQueryLedgers {}

/// Request builder for the Get Trade Volume endpoint
pub struct KITradeVolume {
    params: IndexMap<String, String>,
}

impl KITradeVolume {
    pub fn build() -> Self {
        KITradeVolume {
            params: IndexMap::new()
        }
    }

    pub fn clear_pair_list(self) -> Self {
        self.update_input("pair", String::from(""))
    }

    pub fn with_pair(self, pair: KAssetPair) -> Self {
        self.with_item(pair)
    }

    pub fn with_pair_list<T>(self, pairs: T) -> Self
        where T: IntoIterator<Item = KAssetPair>
    {
        self.with_item_list(pairs)
    }

    pub fn with_fee_info(self, feeinfo: bool) -> Self {
        self.update_input("fee-info", feeinfo.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
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

/// Request builder for the Add Standard Order endpoint
pub struct KIAddOrder {
    params: IndexMap<String, String>,
}

impl KIAddOrder {
    pub fn build(pair: KAssetPair, tradetype: TradeType, ordertype: OrderType, volume: f64) -> Self {
        let new = KIAddOrder {
            params: IndexMap::new()
        };

        new.with_pair(pair)
           .with_transaction_type(tradetype)
           .with_order_type_ref(&ordertype)
           .with_price1(&ordertype)
           .with_price2(&ordertype)
           .with_volume(volume)
    }

    pub fn with_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    pub fn with_transaction_type(self, tradetype: TradeType) -> Self {
        self.update_input("type", tradetype.to_string())
    }

    pub fn with_order_type(self, ordertype: OrderType) -> Self {
        self.update_input("ordertype", ordertype.to_string())
    }

    fn with_order_type_ref(self, ordertype: &OrderType) -> Self {
        self.update_input("ordertype", ordertype.to_string())
    }

    fn with_price1(self, ordertype: &OrderType) -> Self {
        match ordertype.get_price1() {
            Some(price) => {self.update_input("price", price)},
            None => self
        }
    }

    fn with_price2(self, ordertype: &OrderType) -> Self {
        match ordertype.get_price2() {
            Some(price) => {self.update_input("price2", price)},
            None => self
        }
    }

    pub fn with_volume(self, volume: f64) -> Self {
        self.update_input("volume", volume.to_string())
    }

    pub fn with_leverage(self, leverage: (u8, u8)) -> Self {
        self.update_input("leverage", format!("{}:{}", leverage.0, leverage.1))
    }

    pub fn with_order_flags<T>(mut self, flags: T) -> Self
        where T: IntoIterator<Item = OrderFlags>
    {
        let listname = String::from("oflags");
        match self.params.contains_key(&listname) {
            true => {
                flags.into_iter().for_each(|flag| self.format_flag(flag));
                self
            },
            false => {
                let mut iter = flags.into_iter();
                match iter.next() {
                    Some(val) => {
                        self.params.insert(listname, val.to_string());
                        self.with_order_flags(iter)
                    },
                    None => self,
                }
            }
        }
    }

    pub fn start_in(self, secs: u32) -> Self {
        self.update_input("starttm", String::from("%2B") + &secs.to_string())
    }

    pub fn start_at(self, timestamp: u64) -> Self {
        self.update_input("starttm", timestamp.to_string())
    }

    pub fn expire_in(self, secs: u32) -> Self {
        self.update_input("expiretm", secs.to_string())
    }

    pub fn expire_at(self, timestamp: u64) -> Self {
        self.update_input("expiretm", timestamp.to_string())
    }

    pub fn with_userref(self, userref: u32) -> Self {
        self.update_input("userref", userref.to_string())
    }

    pub fn validate(self, validate: bool) -> Self {
        self.update_input("validate", validate.to_string())
    }

    pub fn with_closing_order(self, ordertype: OrderType) -> Self{
        let price1 = ordertype.get_price1();
        let price2 = ordertype.get_price2();
        match (price1, price2) {
            (Some(price1), Some(price2)) => {
                self.update_input("close%5Bordertype%5D", ordertype.to_string())
                    .update_input("close%5Bprice%5D", price1)
                    .update_input("close%5Bprice2%5D", price2)
            },
            (Some(price1), None) => {
                self.update_input("close%5Bordertype%5D", ordertype.to_string())
                    .update_input("close%5Bprice%5D", price1)
            },
            (None, Some(_)) => {
                unreachable!()
            },
            (None, None) => {
                self
            },
        }
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }

    fn format_flag(&mut self, flag: OrderFlags) 
    {
        let listname = String::from("oflags");
        match self.params.get_mut(&listname) {
            Some(list) => {
                // Silently disallow adding the same input to the list multiple times
                if list.contains(&flag.to_string()) {
                    return;
                }

                *list = format!("{},{}", list, flag.to_string());
            },
            None => {
                self.list_mut().insert(listname, flag.to_string());
            },
        }
    }
}
impl MutateInput for KIAddOrder {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KIAddOrder {}

impl Input for KIAddOrder {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("AddOrder") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("AddOrder") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

/// Request builder for the Cancel Open Order endpoint
pub struct KICancelOrder {
    params: IndexMap<String, String>,
}

impl KICancelOrder {
    pub fn build(txid: String) -> KICancelOrder {
        let cancelorder = KICancelOrder {
            params: IndexMap::new()
        };
        cancelorder.with_txid(txid)
    }

    pub fn with_txid(self, txid: String) -> Self {
        self.update_input("txid", txid)
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl MutateInput for KICancelOrder {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KICancelOrder {}

impl Input for KICancelOrder {
    fn finish(self) -> KrakenInput {
        KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("CancelOrder") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("CancelOrder") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}

/// Request builder for the Cancel All Open Orders endpoint
pub struct KICancelAllOrders {
    params: IndexMap<String, String>,
}

impl KICancelAllOrders {
    pub fn build() -> KrakenInput {
        let cancelorders = KICancelAllOrders {
            params: IndexMap::new()
        };
        cancelorders.finish()
    }

    pub fn build_clone() -> (KrakenInput, Self) {
        let cancelorders = KICancelAllOrders {
            params: IndexMap::new()
        };
        cancelorders.finish_clone()
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl Input for KICancelAllOrders {
    fn finish(self) -> KrakenInput {
       KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("CancelAll") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
       info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("CancelAll") },
       params: Some(newself.params.clone())
       },
       newself)
    }
}

impl MutateInput for KICancelAllOrders {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KICancelAllOrders {}

/// Request builder for the Cancel All Orders After endpoint
pub struct KICancelOnTimeout {
    params: IndexMap<String, String>,
}

impl KICancelOnTimeout {
    pub fn build(timeout: u32) -> KICancelOnTimeout {
        let cancelorder = KICancelOnTimeout {
            params: IndexMap::new()
        };
        cancelorder.on_timeout(timeout)
    }

    pub fn on_timeout(self, timeout: u32) -> Self {
        self.update_input("timeout", timeout.to_string())
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }
}

impl MutateInput for KICancelOnTimeout {
    fn list_mut(&mut self) -> &mut IndexMap<String, String> {
        &mut self.params
    }
}

impl UpdateInput for KICancelOnTimeout {}

impl Input for KICancelOnTimeout {
    fn finish(self) -> KrakenInput {
        KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("CancelAllOrdersAfter") },
           params: Some(self.with_nonce().params)
       }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
       let newself = self.with_nonce();
       (KrakenInput {
           info: EndpointInfo { methodtype: MethodType::Private, endpoint: String::from("CancelAllOrdersAfter") },
           params: Some(newself.params.clone())
       },
       newself)
    }
}


/// Response from the Get Account Balance endpoint | See [KIAccountBalance]
#[derive(Deserialize, Serialize, Debug)]
pub struct KOAccountBalance {
    /// Map with the asset as the key and the asset's current balance as the value
    #[serde(flatten)]
    pub balances: HashMap<String, String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradeBalance {
    /// cost basis of open positions
    pub c: String,
    /// equity = trade balance + unrealized net profit/loss
    pub e: String,
    /// equivalent balance (combined balance of all currencies)
    pub eb: String,
    /// margin amount of open positions
    pub m: String,
    /// free margin = equity - initial margin (maximum margin available to open new positions)
    pub mf: String,
    /// margin level = (equity / initial margin) * 100
    pub ml: String,
    /// unrealized net profit/loss of open positions
    pub n: String,
    /// trade balance (combined balance of all equity currencies)
    pub tb: String,
    /// current floating valuation of open positions
    pub v: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KOOrderDescription {
    pub pair: String,
    #[serde(rename = "type")]
    pub tradetype: String,
    pub ordertype: String,
    pub price: String,
    pub price2: String,
    pub leverage: String,
    #[serde(rename = "order")]
    pub desc: String,
    #[serde(rename = "close")]
    pub closedesc: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum KOOrderStatus {
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
pub struct KOOrderInfo {
    /// Referral order transaction id that created this order
    pub refid: Option<String>,
    /// user reference id
    pub userref: Option<u32>,
    /// status of order:
    pub status: KOOrderStatus,
    /// unix timestamp of when order was placed
    pub opentm: f64,
    /// unix timestamp of order start time (or 0 if not set)
    pub starttm: f64,
    /// unix timestamp of order end time (or 0 if not set)
    pub expiretm: f64,
    /// order description info
    pub descr: KOOrderDescription,
    /// volume of order (base currency unless viqc set in oflags)
    pub vol: String,
    /// volume executed (base currency unless viqc set in oflags)
    pub vol_exec: String,
    /// total cost (quote currency unless unless viqc set in oflags)
    pub cost: String,
    /// total fee (quote currency)
    pub fee: String,
    /// average price (quote currency unless viqc set in oflags)
    pub price: String,
    /// stop price (quote currency, for trailing stops)
    pub stopprice: Option<String>,
    /// triggered limit price (quote currency, when limit based order type triggered)
    pub limitprice: Option<String>,
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
    /// array of trade ids related to order (if trades info requested and data available)
    pub trades: Option<Vec<String>>,
    /// unix timestamp of when order was closed. Field only present when calling ClosedOrders
    /// endpoint
    pub closetm: Option<f64>,
    /// additional info on status (if any). Field only present when calling ClosedOrders
    pub reason: Option<String>,
}

/// Open orders
#[derive(Deserialize, Serialize, Debug)]
pub struct KOOpenOrders {
    pub open: HashMap<String, KOOrderInfo>,
}

/// Closed order result
#[derive(Deserialize, Serialize, Debug)]
pub struct KOClosedOrders {
    pub closed: HashMap<String, KOOrderInfo>,
    pub count: u32,
}

/// Orders query results
pub type KOQueriedOrders = HashMap<String, KOOrderInfo>;

#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradeInfo {
    /// Order responsible for execution of trade
    ordertxid: String,
    pair: String,
    time: f64,
    #[serde(rename = "type")]
    tradetype: String,
    ordertype: String,
    price: String,
    cost: String,
    fee: String,
    vol: String,
    margin: Option<String>,
    misc: String,
    posstatus: Option<String>,
    cprice: Option<String>,
    cfee: Option<String>,
    cvol: Option<String>,
    cmargin: Option<String>,
    net: Option<String>,
    trades: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradeHistory {
    pub closed: HashMap<String, KOTradeInfo>,
    pub count: u32,
}

pub type KOQueriedTrades = HashMap<String, KOTradeInfo>;

#[derive(Deserialize, Serialize, Debug)]
pub struct KOPositionInfo {
    /// Order responsible for execution of trade
    ordertxid: String,
    pair: String,
    time: f64,
    #[serde(rename = "type")]
    tradetype: String,
    ordertype: String,
    cost: String,
    fee: String,
    vol: String,
    vol_closed: String,
    margin: Option<String>,
    value: Option<String>,
    net: Option<String>,
    misc: String,
    oflags: Option<String>,
}

pub type KOOpenPositions = HashMap<String, KOPositionInfo>;

#[derive(Deserialize, Serialize, Debug)]
pub struct KOLedgerInfo {
    /// Order responsible for execution of trade
    refid: String,
    time: f64,
    #[serde(rename = "type")]
    ledgertype: String,
    aclass: String,
    asset: String,
    amount: String,
    fee: String,
    balance: Option<String>,
}

pub type KOLedgers = HashMap<String, KOLedgerInfo>;

#[derive(Deserialize, Serialize, Debug)]
pub struct KOFeeInfo {
    fee: String,
    minfee: Option<String>,
    maxfee: Option<String>,
    nextfee: Option<String>,
    nextvolume: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KOMakerFeeInfo {
    fee: String,
    minfee: Option<String>,
    maxfee: Option<String>,
    nextfee: Option<String>,
    nextvolume: Option<String>,
    tiervolume: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KOTradeVolume {
    currency: String,
    volume: String,
    fees: Option<HashMap<String, KOFeeInfo>>,
    fees_maker: Option<HashMap<String, KOMakerFeeInfo>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KOCancelledOrders {
    /// number of orders canceled
    count: u32,
    /// if set, order(s) is/are pending cancellation
    pending: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct KOAddOrder {
    /// Order description info
    descr: AddOrderDesc,
    /// Array of transaction ids for order (if order was added successfully)
    txid: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct AddOrderDesc {
    /// Order description
    order: String,
    /// Conditional close order description (if order was added successfully)
    close: Option<String>,
}
