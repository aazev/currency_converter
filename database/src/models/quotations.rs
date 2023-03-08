use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::symbols::Symbol;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    id: i64,
    symbol_id: i64,
    base_symbol_id: i64,
    date: NaiveDate,
    open: BigDecimal,
    close: BigDecimal,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertableQuotation {
    symbol_id: u64,
    base_symbol_id: u64,
    date: NaiveDateTime,
    open: BigDecimal,
    close: BigDecimal,
}

pub async fn retrieve_quotations(
    symbol: Symbol,
    db_pool: &PgPool,
) -> Result<Vec<Quotation>, Box<dyn std::error::Error>> {
    let mut quotations = Vec::new();
    let rows = sqlx::query_as!(
        Quotation,
        "SELECT * FROM quotations where symbol_id = $1",
        symbol.id
    )
    .fetch_all(db_pool)
    .await?;
    for row in rows {
        quotations.push(row);
    }
    Ok(quotations)
}
