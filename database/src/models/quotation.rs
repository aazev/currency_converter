use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Quotation {
    id: u64,
    symbol_id: u64,
    base_symbol_id: u64,
    date: NaiveDateTime,
    open: f64,
    close: f64,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InsertableQuotation {
    symbol_id: u64,
    base_symbol_id: u64,
    date: NaiveDateTime,
    open: f64,
    close: f64,
}
