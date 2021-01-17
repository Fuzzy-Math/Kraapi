use std::fmt;
use indexmap::map::IndexMap;
use std::fmt::{Debug,Display};

pub mod public;
pub mod private;

pub(crate) enum MethodType {
    PRIVATE,
    PUBLIC,
}

impl fmt::Display for MethodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MethodType::PRIVATE => write!(f, "{}", "private"),
            MethodType::PUBLIC => write!(f, "{}", "public"),
        }
    }
}

pub(crate) struct EndpointInfo {
    methodtype: MethodType,
    endpoint: String,
}

impl EndpointInfo {
    pub(crate) fn get_type(&self) -> &MethodType {
        &self.methodtype
    }
    pub(crate) fn get_endpoint(&self) -> &String {
        &self.endpoint
    } 
}

pub struct KrakenInput {
    info: EndpointInfo,
    params: Option<IndexMap<String, String>>
}

impl KrakenInput {
    pub(crate) fn get_info(&self) -> &EndpointInfo {
        &self.info
    }

    pub(crate) fn get_params(&self) -> Option<&IndexMap<String, String>> {
        match &self.params {
            Some(params) => Some(&params),
            None => None
        }
    }
}

pub trait Input {
    fn finish_input(self) -> KrakenInput;
}

pub(crate) fn format_params<T, U>(params: &Option<&IndexMap<T, U>>) -> Option<String>
    where T: Display,
          U: Display
{
    match params {
        Some(params) => {
            let mut res = String::new();
            for index in 0..params.len() {
                let pair = params.get_index(index).unwrap();
                if index == 0 {
                    res = format!("{}{}={}", res, pair.0, pair.1);
                } else {
                    res = format!("{}&{}={}", res, pair.0, pair.1);
                }
            }
            Some(res)
        },
        None => None
    }
}

