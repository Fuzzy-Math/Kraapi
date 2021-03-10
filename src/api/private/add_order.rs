use indexmap::map::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

use crate::auth::KrakenAuth;
// Structs/Enums
use super::{EndpointInfo, KAssetPair, KrakenInput, MethodType, OrderFlags, OrderType, TradeType};

// Traits
use super::{Input, MutateInput, Output, UpdateInput};

/// Amount of leverage to be used when placing an order
pub enum Leverage {
    /// 2x leverage
    Two,
    /// 3x leverage
    Three,
    /// 4x leverage
    Four,
    /// 5x leverage
    Five
}

impl Display for Leverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Leverage::Two => write!(f, "2"),
            Leverage::Three => write!(f, "3"),
            Leverage::Four => write!(f, "4"),
            Leverage::Five => write!(f, "5"),
        }
    }
}

/// Request builder for the Add Standard Order endpoint
pub struct KIAddOrder {
    params: IndexMap<String, String>,
}

impl KIAddOrder {
    /// Constructor returning a KrakenInput builder for the add standard order endpoint
    ///
    /// * `pair` - asset pair for order
    /// * `tradetype` - [TradeType]
    /// * `ordertype` - [OrderType]
    /// * `volume` - order volume in lots
    pub fn build(
        pair: KAssetPair,
        tradetype: TradeType,
        ordertype: OrderType,
        volume: f64,
    ) -> Self {
        let new = KIAddOrder {
            params: IndexMap::new(),
        };

        new.with_pair(pair)
            .with_transaction_type(tradetype)
            .with_order_type_ref(&ordertype)
            .with_price1(&ordertype)
            .with_price2(&ordertype)
            .with_volume(volume)
    }

    /// Update the asset pair for this order. Useful for templating
    pub fn with_pair(self, pair: KAssetPair) -> Self {
        self.update_input("pair", pair.to_string())
    }

    /// Update the transaction/trade type for this order. Useful for templating
    pub fn with_transaction_type(self, tradetype: TradeType) -> Self {
        self.update_input("type", tradetype.to_string())
    }

    /// Update the order type for this order. Useful for templating. Note that OrderType encodes
    /// the desired price (limit price, stop loss trigger price etc.)
    pub fn with_order_type(self, ordertype: OrderType) -> Self {
        self.update_input("ordertype", ordertype.to_string())
    }

    fn with_order_type_ref(self, ordertype: &OrderType) -> Self {
        self.update_input("ordertype", ordertype.to_string())
    }

    fn with_price1(self, ordertype: &OrderType) -> Self {
        match ordertype.price1() {
            Some(price) => self.update_input("price", price),
            None => self,
        }
    }

    fn with_price2(self, ordertype: &OrderType) -> Self {
        match ordertype.price2() {
            Some(price) => self.update_input("price2", price),
            None => self,
        }
    }

    /// Update the order volume in lots
    pub fn with_volume(self, volume: f64) -> Self {
        self.update_input("volume", volume.to_string())
    }

    /// Amount of leverage for this order. Subject to [margin trading
    /// restrictions](https://support.kraken.com/hc/en-us/articles/227876608)
    pub fn with_leverage(self, leverage: Leverage) -> Self {
        self.update_input("leverage", format!("{}:{}", leverage.to_string(), 1u8))
    }

    /// Order flags to set. Accepts any iterable collection of [OrderFlags]
    pub fn with_order_flags<T>(mut self, flags: T) -> Self
    where
        T: IntoIterator<Item = OrderFlags>,
    {
        let listname = String::from("oflags");
        match self.params.contains_key(&listname) {
            true => {
                flags.into_iter().for_each(|flag| self.format_flag(flag));
                self
            }
            false => {
                let mut iter = flags.into_iter();
                match iter.next() {
                    Some(val) => {
                        self.params.insert(listname, val.to_string());
                        self.with_order_flags(iter)
                    }
                    None => self,
                }
            }
        }
    }

    /// Scedule the order start time for `secs` seconds from now
    pub fn start_in(self, secs: u32) -> Self {
        self.update_input("starttm", String::from("%2B") + &secs.to_string())
    }

    /// Scedule the order start time for the Unix `timestamp` in seconds
    pub fn start_at(self, timestamp: u64) -> Self {
        self.update_input("starttm", timestamp.to_string())
    }

    /// Order to expire in `secs` seconds
    pub fn expire_in(self, secs: u32) -> Self {
        self.update_input("expiretm", secs.to_string())
    }

    /// Order to expire at the Unix `timestamp` in seconds
    pub fn expire_at(self, timestamp: u64) -> Self {
        self.update_input("expiretm", timestamp.to_string())
    }

    /// User supplied unsigned 32 bit integer which Kraken will use to demarcate this order for
    /// future reference
    pub fn with_userref(self, userref: u32) -> Self {
        self.update_input("userref", userref.to_string())
    }

    /// Validate inputs on Kraken's servers. Don't submit order
    pub fn validate(self, validate: bool) -> Self {
        self.update_input("validate", validate.to_string())
    }

    /// Closing order to add to the system when this order gets filled
    pub fn with_closing_order(self, ordertype: OrderType) -> Self {
        let price1 = ordertype.price1();
        let price2 = ordertype.price2();
        match (price1, price2) {
            (Some(price1), Some(price2)) => self
                .update_input("close%5Bordertype%5D", ordertype.to_string())
                .update_input("close%5Bprice%5D", price1)
                .update_input("close%5Bprice2%5D", price2),
            (Some(price1), None) => self
                .update_input("close%5Bordertype%5D", ordertype.to_string())
                .update_input("close%5Bprice%5D", price1),
            (None, Some(_)) => {
                unreachable!()
            }
            (None, None) => self,
        }
    }

    fn with_nonce(self) -> Self {
        self.update_input("nonce", KrakenAuth::nonce())
    }

    fn format_flag(&mut self, flag: OrderFlags) {
        let listname = String::from("oflags");
        match self.params.get_mut(&listname) {
            Some(list) => {
                // Silently disallow adding the same input to the list multiple times
                if list.contains(&flag.to_string()) {
                    return;
                }

                *list = format!("{},{}", list, flag.to_string());
            }
            None => {
                self.list_mut().insert(listname, flag.to_string());
            }
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
            info: EndpointInfo {
                methodtype: MethodType::Private,
                endpoint: String::from("AddOrder"),
            },
            params: Some(self.with_nonce().params),
        }
    }

    fn finish_clone(self) -> (KrakenInput, Self) {
        let newself = self.with_nonce();
        (
            KrakenInput {
                info: EndpointInfo {
                    methodtype: MethodType::Private,
                    endpoint: String::from("AddOrder"),
                },
                params: Some(newself.params.clone()),
            },
            newself,
        )
    }
}

/// Response from the Add Standard Order endpoint
#[derive(Deserialize, Serialize, Debug)]
pub struct KOAddOrder {
    /// Order description info
    pub descr: AddOrderDesc,
    /// Array of transaction ids for order (if order was added successfully)
    pub txid: Option<Vec<String>>,
}

impl Output for KOAddOrder {}

/// Textual description of placed order and optional close order
#[derive(Deserialize, Serialize, Debug)]
pub struct AddOrderDesc {
    /// Order description
    pub order: String,
    /// Conditional close order description (if order was added successfully)
    pub close: Option<String>,
}
