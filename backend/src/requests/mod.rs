use serde::{Deserialize, Serialize};

const BASE_SYMBOL: &str = "EUR";
const SYMBOLS_URL: &str = "https://api.apilayer.com/fixer/symbols";
const FLUCTUATIONS_URL: &str =
    "https://api.apilayer.com/fixer/fluctuation?base={}&start_date={}&end_date={}";
const QUOTATIONS_URL: &str =
    "https://api.apilayer.com/fixer/timeseries?base={}&symbols={}&start_date={}&end_date={}";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluctuationsRequest {
    base: String,
    start_date: String,
    end_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuotationsRequest {
    base: String,
    start_date: String,
    end_date: String,
}
