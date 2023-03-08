use serde::{Deserialize, Serialize};

pub static BASE_SYMBOL: &str = "EUR";
pub const SYMBOLS_URL: &str = "https://api.apilayer.com/exchangerates_data/symbols";
pub const FLUCTUATIONS_URL: &str =
    "https://api.apilayer.com/exchangerates_data/fluctuation?base={}&start_date={}&end_date={}";
pub const QUOTATIONS_URL: &str =
    "https://api.apilayer.com/exchangerates_data/timeseries?base={}&symbols={}&start_date={}&end_date={}";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluctuationsRequest {
    pub base: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuotationsRequest {
    pub base: String,
    pub start_date: String,
    pub end_date: String,
}