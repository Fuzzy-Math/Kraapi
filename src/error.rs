//! Module encapsulating the error handling of internal errors and errors returned from Kraken

use std::error::Error;
use std::fmt;

use hyper::Error as HyperError;
use serde_json::Error as SerdeError;

/// Possible errors that could occur internally or errors that were returned from Kraken
#[derive(Debug)]
pub enum KrakenError {
    /// Wrapper around [hyper::Error] when an internal http error has occurred
    HttpError(HyperError),

    /// Wrapper around [serde_json::Error] for when serde fails to parse the json into the output
    /// structure
    ParseError(SerdeError),

    /// Invalid currency pair
    /// You can pull the complete list of our asset pairs from the AssetPairs public call
    /// and look for the pair name as the entry of the Json headers or by the parameter
    /// "altname": `https://api.kraken.com/0/public/AssetPairs`
    UnknownAssetPair,

    /// This error is returned when a method is called without the required parameters.
    /// For example, calling the QueryOrders method without specifying a valid transaction
    /// id (txid) parameter would cause the invalid arguments error to be returned.
    /// Calling a method with unnecessary parameters would still not return the
    /// invalid arguments error because the unnecessary parameters would simply be ignored.
    InvalidArguments,

    /// Permission denied errors are returned when the API client is attempting a task
    /// for which the API key does not have permission. For example, if an API client
    /// attempted to retrieve the account balance using an API key that was configured to
    /// allow trading access but not account management access, then the permission denied
    /// error would be returned. You can review your API keys and their settings
    /// (such as their permissions) via the Settings -> API tab of account management.
    PermissionDenied,

    /// This error is returned when the API key used for the call is either expired or disabled,
    /// please review the API key in your Settings -> API tab of account management or
    /// generate a new one and update your application.
    InvalidKey,

    /// The Invalid Key error occurs if either your API key or API secret are written
    /// incorrectly in your program or because the POST data used in the authentication
    /// and the POST data sent to the API do not match
    InvalidSignature,

    /// This error is returned when an invalid nonce is sent.
    /// Check your [nonce
    /// window](https://support.kraken.com/hc/en-us/articles/360001148023-What-is-a-Nonce-Window-)
    InvalidNonce,

    /// This error occurs when the [API call
    /// limits](https://support.kraken.com/hc/en-us/articles/206548367-What-is-the-API-call-rate-limit-)
    /// are exceeded
    APIRateLimit,

    /// While adding/canceling orders does not count against our standard API counter limits,
    /// these operations do have their own add/cancel order counter. This counter works in a
    /// way where the longer orders are left on the book, the more orders clients are able
    /// to add/cancel. After the error "EAPI:Rate limit exceeded", please wait ~15 min for
    /// being able to send new requests.
    OrderRateLimit,

    /// Temporary lockout error messages can occur if you had too many failed API calls
    /// or too many invalid nonce errors in a short period of time or invalid signatures.
    /// Even though these calls return an error, that error still counts against your API
    /// limits and may result in a temporary lockout.
    ///
    /// Temporary lockouts typically last approximately 15 minutes. If you are triggering
    /// several invalid nonce errors, please increase the nonce window as this can help
    /// reduce the frequency that these errors will occur. Please try to reduce the frequency
    /// of your private API calls also.
    TemporaryLockout,

    /// Opening new spot positions on margin has been temporarily suspended for trading
    /// engine maintenance. The feature will be making a return soon and you can follow
    /// along with [updates](status.kraken.com)
    ///
    /// Another reasons may be that spot positions on margin are not currently available
    /// for clients residing in certain countries. Please see this article for our
    /// [geographical restrictions](https://support.kraken.com/hc/en-us/articles/360001368823)
    OpenPosition,

    /// No [hedging](https://support.kraken.com/hc/en-us/articles/205367328-Hedging).
    /// Cannot open a long and short position for the same pair.
    ///
    /// If wishing to open a long and short position for the same currency, please
    /// choose different trading pairs with the same currency as the base or quote currency.
    /// Ex: short XBT/USD, long XBT/EUR.
    OpposingPosition,

    /// This error occurs when you have exceeded the margin allowance limits for your
    /// current verification level. Margin allowance limits for each currency varies
    /// based on your current verification level. Please refer to this support article for
    /// more information regarding [margin allowance limits](https://support.kraken.com/hc/en-us/articles/209238787-Margin-allowance-limits)
    MarginAllowanceExceeded,

    /// We have limited funds available for margin extensions. The "insufficient margin"
    /// message indicates that we are out of funds in the applicable margin pool for the
    /// time being. This can change at any time. You may be able to successfully place your
    /// order just seconds or minutes later, but high volume orders and orders placed during
    /// high volume times may take longer. Please accept our apologies for any inconvenience.
    /// For more [information](https://support.kraken.com/hc/en-us/articles/217696017-Insufficient-Margin)
    InsufficientMargin,

    /// You do not have the funds available to place this order. Please review your open
    /// positions and orders for items that may be holding up your funds
    InsufficientFunds,

    /// You have not met the minimum order volume for this asset.
    ///
    /// You can find more information about [minimum order sizes](https://support.kraken.com/hc/en-us/articles/205893708-What-is-the-minimum-order-size-volume-)
    OrderMinimum,

    /// You have exceeded the maximum amount of open orders available to your account.
    ///
    /// These limits are based on your verification level. Please close some of your open
    /// orders or verify your account to a higher level.
    ///
    /// You can learn more about the [maximum amount of open orders](https://support.kraken.com/hc/en-us/articles/209090607-What-is-the-maximum-number-of-open-orders-positions-)
    OrderLimit,

    /// You have exceeded the maximum amount of open positions available to your account.
    ///
    /// These limits are based on your verification level. Please close or settle some or all
    /// of your open positions or verify your account to a higher level if possible.
    ///
    /// You can learn more about the [maximum amount of open positions](https://support.kraken.com/hc/en-us/articles/209090607-What-is-the-maximum-number-of-open-orders-positions-)
    PositionLimit,

    /// In case of this error you will need to submit your order with the following parameter:
    /// ‘trading_agreement’:’agree’
    ///
    /// This will resolve the error message you are receiving when placing an order:
    /// [Trading Agreement](https://support.kraken.com/hc/en-us/articles/360000920026-Trading-Agreement-required-for-orders-sent-via-API)
    TradingAgreement,

    /// The service errors you are experiencing should only be temporary. You may wish to
    /// resubmit your requests if they have failed. We will be monitoring the issues and
    /// will update our [page](https://status.kraken.com/)
    ServiceUnavailable,

    /// The service errors you are experiencing should only be temporary. You may wish to
    /// resubmit your requests if they have failed. We will be monitoring the issues and
    /// will update our [page](https://status.kraken.com/)
    ServiceBusy,

    /// When we are facing API degradation issues, these can translate into problems for
    /// both Kraken and cryptowat.ch in the form of service unavailable messages, 8XX errors
    /// on [cryptowatch](https://cryptowat.ch/) and site outages.
    InternalError,

    /// This issue has to do with the security of your account which may have been compromised.
    /// Please change your password and Two-Factor Authentication and contact our Support Center
    Locked,

    /// This error occurs when a flag or input parameter is disabled temporary or permanently.
    /// The error should come from one of the inputs passed, please contact our support sending
    /// a log with the complete informations used for the call that generated the error
    FeatureDisabled,

    /// Default KrakenError if none of the others above
    UnknownError,
}

impl Error for KrakenError {}

impl fmt::Display for KrakenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KrakenError::HttpError(err) => write!(f, "HTTP Error: {}", err.to_string()),
            KrakenError::ParseError(err) => write!(f, "Parse Error: {}", err.to_string()),

            KrakenError::UnknownAssetPair => write!(f, "Unknown AssetPair"),
            KrakenError::InvalidArguments => write!(f, "Invalid Arguments"),
            KrakenError::PermissionDenied => write!(f, "Permission Denied"),
            KrakenError::InvalidKey => write!(f, "Invalid Key"),
            KrakenError::InvalidSignature => write!(f, "Invalid Signature"),
            KrakenError::InvalidNonce => write!(f, "Invalid Nonce"),
            KrakenError::APIRateLimit => write!(f, "API Rate Limit"),
            KrakenError::OrderRateLimit => write!(f, "Order Rate Limit"),
            KrakenError::TemporaryLockout => write!(f, "Temporary Lockout"),
            KrakenError::OpenPosition => write!(f, "Cannot Open Position"),
            KrakenError::OpposingPosition => write!(f, "Cannot Open Opposing Position"),
            KrakenError::MarginAllowanceExceeded => write!(f, "Margin Allowance Exceeded"),
            KrakenError::InsufficientMargin => write!(f, "Insufficient Margin"),
            KrakenError::InsufficientFunds => write!(f, "Insufficient User Funds"),
            KrakenError::OrderMinimum => write!(f, "Order Minimum Not Met (volume too low)"),
            KrakenError::OrderLimit => write!(f, "Orders Limit Reached"),
            KrakenError::PositionLimit => write!(f, "Positions Limit Reached"),
            KrakenError::TradingAgreement => write!(f, "Trading Agreement Required"),
            KrakenError::ServiceUnavailable => write!(f, "Service Unavailable"),
            KrakenError::ServiceBusy => write!(f, "Service Busy"),
            KrakenError::InternalError => write!(f, "Internal Error"),
            KrakenError::Locked => write!(f, "Account Locked"),
            KrakenError::FeatureDisabled => write!(f, "A Feature Was Disabled"),
            KrakenError::UnknownError => write!(f, "An Unknown Error Occurred"),
        }
    }
}

impl From<HyperError> for KrakenError {
    fn from(err: HyperError) -> Self {
        KrakenError::HttpError(err)
    }
}

impl From<SerdeError> for KrakenError {
    fn from(err: SerdeError) -> Self {
        KrakenError::ParseError(err)
    }
}

impl From<KrakenError> for Vec<KrakenError> {
    fn from(err: KrakenError) -> Self {
        vec![err]
    }
}

pub(crate) fn generate_errors(errors: Vec<String>) -> Vec<KrakenError> {
    let mut ret: Vec<KrakenError> = Vec::with_capacity(errors.len());
    for error in errors {
        let index = error.find(':').unwrap();
        // Assume kraken will not return an error like "EError:" with no error description
        let (category, message) = error.split_at(index + 1);

        let err: KrakenError = match message {
            "Unknown asset pair" => KrakenError::UnknownAssetPair,
            "Invalid arguments" => KrakenError::InvalidArguments,
            "Permission denied" => KrakenError::PermissionDenied,
            "Invalid key" => KrakenError::InvalidKey,
            "Invalid signature" => KrakenError::InvalidSignature,
            "Invalid nonce" => KrakenError::InvalidNonce,
            "Rate limit exceeded" => match category {
                "EAPI:" => KrakenError::APIRateLimit,
                "EOrder:" => KrakenError::OrderRateLimit,
                _ => KrakenError::UnknownError,
            },
            "Temporary lockout" => KrakenError::TemporaryLockout,
            "Cannot open position" => KrakenError::OpenPosition,
            "Cannot open opposing position" => KrakenError::OpposingPosition,
            "Margin allowance exceeded" => KrakenError::MarginAllowanceExceeded,
            "Insufficient margin" => KrakenError::InsufficientMargin,
            "Insufficient insufficient user funds" => KrakenError::InsufficientFunds,
            "Order minimum not volume too low" => KrakenError::OrderMinimum,
            "Orders limit exceeded" => KrakenError::OrderLimit,
            "Positions limit exceeded" => KrakenError::PositionLimit,
            "Trading agreement required" => KrakenError::TradingAgreement,
            "Unavailable" => KrakenError::ServiceUnavailable,
            "Busy" => KrakenError::ServiceBusy,
            "Internal error" => KrakenError::InternalError,
            "Locked" => KrakenError::Locked,
            "Feature disabled" => KrakenError::FeatureDisabled,
            _ => KrakenError::UnknownError,
        };

        ret.push(err);
    }

    ret
}
