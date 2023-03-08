use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SymbolsResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub symbols: Map<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuotationResponse {
    pub success: bool,
    pub timeseries: bool,
    pub start_date: String,
    pub end_date: String,
    pub base: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub rates: Map<String, Map<String, f64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluctuationResponse {
    pub success: bool,
    pub fluctuation: bool,
    pub start_date: String,
    pub end_date: String,
    pub base: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub rates: Map<String, Map<String, f64>>,
}
