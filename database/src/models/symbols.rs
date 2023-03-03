use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Symbol {
    id: u64,
    code: String,
    name: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InsertableSymbol {
    code: String,
    name: String,
}
