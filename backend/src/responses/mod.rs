use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SymbolsResponse {
    success: bool,
    #[serde(skip_serializing_if = "Map::is_empty")]
    symbols: Map<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuotationResponse {
    success: bool,
    timeseries: bool,
    start_date: String,
    end_date: String,
    base: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    rates: Map<String, Map<String, f64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluctuationResponse {
    success: bool,
    fluctuation: bool,
    start_date: String,
    end_date: String,
    base: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    rates: Map<String, Map<String, f64>>,
}
