//! Module containing the definitions and processing of supported assets on the Kraken exchange

use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use serde::de::Deserializer;

use crate::error::{KError, KrakenErrors};

// TODO: Query AssetInfo endpoint and write script to fill out the
// enum and trait impl
/// Assets accepted on the Kraken Exchange
/// # FIXME
/// Basic currencies used for testing. Open pull request to add more currencies <https://github.com/Fuzzy-Math/KrakenAPI-Rust>
#[derive(Serialize, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub enum KAsset {
    AAVE,
    ADA,
    ALGO,
    ANT,
    ATOM,
    AtomS,
    /// Australian Dollar
    AUD,
    BAL,
    BAT,
    BCH,
    /// Canadian Dollar
    CAD,
    CHF,
    COMP,
    CRV,
    DAI,
    DASH,
    DOT,
    DotS,
    EOS,
    ETC,
    ETH,
    ETH2,
    Eth2S,
    /// Euro
    EUR,
    EurHold,
    EurM,
    EWT,
    FIL,
    FLOW,
    FLOWH,
    FlowhS,
    FlowS,
    GBP,
    GNO,
    GRT,
    ICX,
    JPY,
    KAVA,
    KavaS,
    KEEP,
    KFEE,
    KNC,
    KSM,
    KsmS,
    LINK,
    LSK,
    LTC,
    MANA,
    MLN,
    NANO,
    OCEAN,
    OMG,
    OXT,
    PAXG,
    QTUM,
    REP,
    REPV2,
    SC,
    SNX,
    STORJ,
    TBTC,
    TRX,
    UNI,
    USDC,
    USDT,
    /// United States Dollar
    USD,
    UsdHold,
    UsdM,
    WAVES,
    /// Bitcoin
    XBT,
    XbtM,
    XDG,
    XLM,
    XMR,
    /// Ripple
    XRP,
    XTZ,
    XtzS,
    YFI,
    ZEC,
}

impl Display for KAsset {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> 
        std::fmt::Result
    {
        match self {
            KAsset::AAVE => write!(f, "AAVE"),
            KAsset::ADA => write!(f, "ADA"),
            KAsset::ALGO => write!(f, "ALGO"),
            KAsset::ANT => write!(f, "ANT"),
            KAsset::ATOM => write!(f, "ATOM"),
            KAsset::AtomS => write!(f, "ATOM.S"),
            KAsset::AUD => write!(f, "AUD"),
            KAsset::BAL => write!(f, "BAL"),
            KAsset::BAT => write!(f, "BAT"),
            KAsset::BCH => write!(f, "BCH"),
            KAsset::CAD => write!(f, "CAD"),
            KAsset::CHF => write!(f, "CHF"),
            KAsset::COMP => write!(f, "COMP"),
            KAsset::CRV => write!(f, "CRV"),
            KAsset::DAI => write!(f, "DAI"),
            KAsset::DASH => write!(f, "DASH"),
            KAsset::DOT => write!(f, "DOT"),
            KAsset::DotS => write!(f, "DOT.S"),
            KAsset::EOS => write!(f, "EOS"),
            KAsset::ETC => write!(f, "ETC"),
            KAsset::ETH => write!(f, "ETH"),
            KAsset::ETH2 => write!(f, "ETH2"),
            KAsset::Eth2S => write!(f, "ETH2.S"),
            KAsset::EUR => write!(f, "EUR"),
            KAsset::EurHold => write!(f, "EUR.HOLD"),
            KAsset::EurM => write!(f, "EUR.M"),
            KAsset::EWT => write!(f, "EWT"),
            KAsset::FIL => write!(f, "FIL"),
            KAsset::FLOW => write!(f, "FLOW"),
            KAsset::FLOWH => write!(f, "FLOWH"),
            KAsset::FlowhS => write!(f, "FLOWH.S"),
            KAsset::FlowS => write!(f, "FLOW.S"),
            KAsset::GBP => write!(f, "GBP"),
            KAsset::GNO => write!(f, "GNO"),
            KAsset::GRT => write!(f, "GRT"),
            KAsset::ICX => write!(f, "ICX"),
            KAsset::JPY => write!(f, "JPY"),
            KAsset::KAVA => write!(f, "KAVA"),
            KAsset::KavaS => write!(f, "KAVA.S"),
            KAsset::KEEP => write!(f, "KEEP"),
            KAsset::KFEE => write!(f, "KFEE"),
            KAsset::KNC => write!(f, "KNC"),
            KAsset::KSM => write!(f, "KSM"),
            KAsset::KsmS => write!(f, "KSM.S"),
            KAsset::LINK => write!(f, "LINK"),
            KAsset::LSK => write!(f, "LSK"),
            KAsset::LTC => write!(f, "LTC"),
            KAsset::MANA => write!(f, "MANA"),
            KAsset::MLN => write!(f, "MLN"),
            KAsset::NANO => write!(f, "NANO"),
            KAsset::OCEAN => write!(f, "OCEAN"),
            KAsset::OMG => write!(f, "OMG"),
            KAsset::OXT => write!(f, "OXT"),
            KAsset::PAXG => write!(f, "PAXG"),
            KAsset::QTUM => write!(f, "QTUM"),
            KAsset::REP => write!(f, "REP"),
            KAsset::REPV2 => write!(f, "REPV2"),
            KAsset::SC => write!(f, "SC"),
            KAsset::SNX => write!(f, "SNX"),
            KAsset::STORJ => write!(f, "STORJ"),
            KAsset::TBTC => write!(f, "TBTC"),
            KAsset::TRX => write!(f, "TRX"),
            KAsset::UNI => write!(f, "UNI"),
            KAsset::USDC => write!(f, "USDC"),
            KAsset::USDT => write!(f, "USDT"),
            KAsset::USD => write!(f, "USD"),
            KAsset::UsdHold => write!(f, "USD.HOLD"),
            KAsset::UsdM => write!(f, "USD.M"),
            KAsset::WAVES => write!(f, "WAVES"),
            KAsset::XBT => write!(f, "XBT"),
            KAsset::XbtM => write!(f, "XBT.M"),
            KAsset::XDG => write!(f, "XDG"),
            KAsset::XLM => write!(f, "XLM"),
            KAsset::XMR => write!(f, "XMR"),
            KAsset::XRP => write!(f, "XRP"),
            KAsset::XTZ => write!(f, "XTZ"),
            KAsset::XtzS => write!(f, "XTZ.S"),
            KAsset::YFI => write!(f, "YFI"),
            KAsset::ZEC => write!(f, "ZEC"),
        }
    }
}

// Needed for string to KAsset/KAssetPair conversion with serde
impl FromStr for KAsset {
    type Err = KrakenErrors<KError>;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val.to_uppercase().as_str() {
            // Assets that sometimes use their currency type prefix
            "AUD" | "ZAUD" => Ok(KAsset::AUD),
            "CAD" | "ZCAD" => Ok(KAsset::CAD),
            "EUR" | "ZEUR" => Ok(KAsset::EUR),
            "USD" | "ZUSD" => Ok(KAsset::USD),
            "GBP" | "ZGBP" => Ok(KAsset::GBP),
            "JPY" | "ZJPY" => Ok(KAsset::JPY),
            "XBT" | "XXBT" => Ok(KAsset::XBT),
            "ETC" | "XETC" => Ok(KAsset::ETC),
            "ETH" | "XETH" => Ok(KAsset::ETH),
            "LTC" | "XLTC" => Ok(KAsset::LTC),
            "MLN" | "XMLN" => Ok(KAsset::MLN),
            "REP" | "XREP" => Ok(KAsset::REP),
            "XDG" | "XXDG" => Ok(KAsset::XDG),
            "XLM" | "XXLM" => Ok(KAsset::XLM),
            "XMR" | "XXMR" => Ok(KAsset::XMR),
            "XRP" | "XXRP" => Ok(KAsset::XRP),
            "ZEC" | "XZEC" => Ok(KAsset::ZEC),
            "AAVE" => Ok(KAsset::AAVE),
            "ADA" => Ok(KAsset::ADA),
            "ALGO" => Ok(KAsset::ALGO),
            "ANT" => Ok(KAsset::ANT),
            "ATOM" => Ok(KAsset::ATOM),
            "ATOM.S" => Ok(KAsset::AtomS),
            "BAL" => Ok(KAsset::BAL),
            "BAT" => Ok(KAsset::BAT),
            "BCH" => Ok(KAsset::BCH),
            "CHF" => Ok(KAsset::CHF),
            "COMP" => Ok(KAsset::COMP),
            "CRV" => Ok(KAsset::CRV),
            "DAI" => Ok(KAsset::DAI),
            "DASH" => Ok(KAsset::DASH),
            "DOT" => Ok(KAsset::DOT),
            "DOT.S" => Ok(KAsset::DotS),
            "EOS" => Ok(KAsset::EOS),
            "ETH2" => Ok(KAsset::ETH2),
            "ETH2.S" => Ok(KAsset::Eth2S),
            "EUR.HOLD" => Ok(KAsset::EurHold),
            "EUR.M" => Ok(KAsset::EurM),
            "EWT" => Ok(KAsset::EWT),
            "FIL" => Ok(KAsset::FIL),
            "FLOW" => Ok(KAsset::FLOW),
            "FLOWH" => Ok(KAsset::FLOWH),
            "FLOWH.S" => Ok(KAsset::FlowhS),
            "FLOW.S" => Ok(KAsset::FlowS),
            "GNO" => Ok(KAsset::GNO),
            "GRT" => Ok(KAsset::GRT),
            "ICX" => Ok(KAsset::ICX),
            "KAVA" => Ok(KAsset::KAVA),
            "KAVA.S" => Ok(KAsset::KavaS),
            "KEEP" => Ok(KAsset::KEEP),
            "KFEE" => Ok(KAsset::KFEE),
            "KNC" => Ok(KAsset::KNC),
            "KSM" => Ok(KAsset::KSM),
            "KSM.S" => Ok(KAsset::KsmS),
            "LINK" => Ok(KAsset::LINK),
            "LSK" => Ok(KAsset::LSK),
            "MANA" => Ok(KAsset::MANA),
            "NANO" => Ok(KAsset::NANO),
            "OCEAN" => Ok(KAsset::OCEAN),
            "OMG" => Ok(KAsset::OMG),
            "OXT" => Ok(KAsset::OXT),
            "PAXG" => Ok(KAsset::PAXG),
            "QTUM" => Ok(KAsset::QTUM),
            "REPV2" => Ok(KAsset::REPV2),
            "SC" => Ok(KAsset::SC),
            "SNX" => Ok(KAsset::SNX),
            "STORJ" => Ok(KAsset::STORJ),
            "TBTC" => Ok(KAsset::TBTC),
            "TRX" => Ok(KAsset::TRX),
            "UNI" => Ok(KAsset::UNI),
            "USDC" => Ok(KAsset::USDC),
            "USDT" => Ok(KAsset::USDT),
            "USD.HOLD" => Ok(KAsset::UsdHold),
            "USD.M" => Ok(KAsset::UsdM),
            "WAVES" => Ok(KAsset::WAVES),
            "XBT.M" => Ok(KAsset::XbtM),
            "XTZ" => Ok(KAsset::XTZ),
            "XTZ.S" => Ok(KAsset::XtzS),
            "YFI" => Ok(KAsset::YFI),
            _     => Err(KrakenErrors(vec![KError::AssetParseError])),
        }
    }
}

impl Debug for KAsset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<'de> Deserialize<'de> for KAsset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where D: Deserializer<'de>
    {
        let buf = String::deserialize(deserializer)?;

        FromStr::from_str(&buf).map_err(serde::de::Error::custom)
    }
}

/// Tradeable asset pair
/// # FIXME
/// Kraken only accepts certain asset pairs as listed from the asset pair endpoint.
/// This data probably should be parsed into a lookup table to ensure only support pairs are
/// accepted.
/// Open a pull request at <https://github.com/Fuzzy-Math/KrakenAPI-Rust>
#[derive(Serialize, Hash, Clone, Copy)]
pub struct KAssetPair(
    //#[serde(deserialize_with = "deserialize_asset")]
    pub KAsset, 
    //#[serde(deserialize_with = "deserialize_asset")]
    pub KAsset
);

impl Display for KAssetPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0.to_string(), self.1.to_string())
    }
}

impl Debug for KAssetPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}{}\"", self.0.to_string(), self.1.to_string())
    }
}

impl PartialEq for KAssetPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for KAssetPair {}

impl FromStr for KAssetPair {
    type Err = KrakenErrors<KError>;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        // Take in a &str, say "XXBTZUSD"
        // Drop the currency type characters at index 0 and 4 ('Z' = fiat, 'X' = crypto)
        // Now we have "XBTUSD"
        // Call from_str() to parse into the two assets
        // Form a KAssetPair tuple from the two returned KAssets
        //Ok(KAssetPair((&val[1..4]).parse::<KAsset>()?, (&val[5..8]).parse::<KAsset>()?))

        match val.len() {
            // We know this is pairs with KAsset::SC as the base currency. A 2/3 split
            5 => {
                println!("base: {}, quote: {}\n", (&val[..2]), (&val[2..]));
                Ok(KAssetPair((&val[..2]).parse::<KAsset>()?, (&val[2..]).parse::<KAsset>()?))
            },

            // Has to be split 3/3. It can't be split 2/4 since that would imply SC is the base
            // currency but we know all pairs with SC are of length 5
            6 => {
                println!("base: {}, quote: {}\n", (&val[..3]), (&val[3..]));
                Ok(KAssetPair((&val[..3]).parse::<KAsset>()?, (&val[3..]).parse::<KAsset>()?))
            },

            // More the likely split 4/3. If that fails to parse, split it 3/4 and parse again
            7 => {
                if let (Ok(base), Ok(quote)) = 
                    (
                        (&val[..4]).parse::<KAsset>(), 
                        (&val[4..]).parse::<KAsset>()
                    )
                {
                    println!("base: {}, quote: {}\n", &val[..4], &val[4..]);
                    Ok(KAssetPair(base, quote))
                } else {
                    if let (Ok(base), Ok(quote)) = 
                        (
                            (&val[..3]).parse::<KAsset>(), 
                            (&val[3..]).parse::<KAsset>()
                        )
                    {
                        Ok(KAssetPair(base, quote))
                    } else {
                        println!("Length 7: {}", val);
                        println!("base: {}, quote: {}\n", &val[..3], &val[3..]);
                        Err(KrakenErrors(vec![KError::AssetParseError]))
                    }
                }
            },

            // Here is the tough one. Probably split 4/4. Else try parsing a 5/3 split. Else try parsing
            // a 3/5 split
            8 => {
                if let (Ok(base), Ok(quote)) = 
                    (
                        (&val[..4]).parse::<KAsset>(), 
                        (&val[4..]).parse::<KAsset>()
                    )
                {
                    println!("base: {}, quote: {}\n", &val[..4], &val[4..]);
                    Ok(KAssetPair(base, quote))
                } else { 
                    if let (Ok(base), Ok(quote)) = 
                        (
                            (&val[..5]).parse::<KAsset>(), 
                            (&val[5..]).parse::<KAsset>()
                        )
                    {
                        println!("base: {}, quote: {}\n", &val[..5], &val[5..]);
                        Ok(KAssetPair(base, quote))
                    } else {
                        if let (Ok(base), Ok(quote)) =
                            (
                                (&val[..3]).parse::<KAsset>(),
                                (&val[3..]).parse::<KAsset>()
                            )
                        {
                            Ok(KAssetPair(base, quote))
                        } else {
                            println!("Length 8: {}", val);
                            println!("base: {}, quote: {}\n", &val[..3], &val[3..]);
                            Err(KrakenErrors(vec![KError::AssetParseError]))
                        }
                    }
                }
            },

            // Length 9 we know to be ETH2_SETH
            9 => Ok(KAssetPair(KAsset::Eth2S, KAsset::ETH)),

            // Don't really know what the pairs that end in ".d" are
            // Just going to chop of the ".d" and pass it back recursively into the parser
            10 => {
                (&val[..8]).parse::<KAssetPair>()                
            }
            // We don't know what we got, Kraken probably changed their api if we are hitting this
            _ => {
                println!("Unknown pair: {}\n", &val);
                Err(KrakenErrors(vec![KError::UnknownAssetPair]))
            },
        }
    }
}

impl TryFrom<&str> for KAssetPair {
    type Error = KrakenErrors<KError>;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        FromStr::from_str(&val)
    }
}

impl TryFrom<String> for KAssetPair {
    type Error = KrakenErrors<KError>;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        FromStr::from_str(&val)
    }
}

impl<'de> Deserialize<'de> for KAssetPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where D: Deserializer<'de>
    {
        let buf = String::deserialize(deserializer)?;

        FromStr::from_str(&buf).map_err(serde::de::Error::custom)
    }
}

/// Asset pair info to retreive | See [KIAssetPairs][public::asset_pairs::KIAssetPairs]
pub enum AssetPairInfo {
    Info,
    Leverage,
    Fees,
    Margin,
}

impl Display for AssetPairInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetPairInfo::Info => write!(f, "info"),
            AssetPairInfo::Leverage => write!(f, "leverage"),
            AssetPairInfo::Fees => write!(f, "fees"),
            AssetPairInfo::Margin => write!(f, "margin"),
        }
    }
}

