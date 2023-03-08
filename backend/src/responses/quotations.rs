use database::models::quotations::Quotation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotationsResponse {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub rates: Vec<Quotation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotationResponse {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub symbol: Quotation,
}
