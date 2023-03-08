use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RabbitMQRequestType {
    Quotation,
    Fluctuation,
    Symbols,
}

impl FromStr for RabbitMQRequestType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "quotation" => Ok(RabbitMQRequestType::Quotation),
            "fluctuation" => Ok(RabbitMQRequestType::Fluctuation),
            "symbols" => Ok(RabbitMQRequestType::Symbols),
            _ => Err(format!("{} is not a valid request type", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RabbitMQRequest {
    pub date_query: NaiveDateTime,
    pub date_start: Option<NaiveDate>,
    pub date_end: Option<NaiveDate>,
    pub base_symbol: Option<String>,
    pub request_type: RabbitMQRequestType,
}
