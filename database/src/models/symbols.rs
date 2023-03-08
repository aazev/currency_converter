use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Symbol {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertableSymbol {
    pub code: String,
    pub name: String,
}

pub async fn retrieve_all_symbols(
    db_pool: &PgPool,
) -> Result<Vec<Symbol>, Box<dyn std::error::Error>> {
    let mut symbols = Vec::new();
    let rows = sqlx::query_as!(Symbol, "SELECT * FROM symbols")
        .fetch_all(db_pool)
        .await?;
    for row in rows {
        symbols.push(row);
    }
    Ok(symbols)
}

pub async fn retrieve_symbol(
    id: i64,
    db_pool: &PgPool,
) -> Result<Symbol, Box<dyn std::error::Error>> {
    let symbol = sqlx::query_as!(Symbol, "SELECT * FROM symbols WHERE id = $1", id)
        .fetch_one(db_pool)
        .await?;
    Ok(symbol)
}

pub async fn retrieve_symbol_by_code(
    code: &str,
    db_pool: &PgPool,
) -> Result<Symbol, Box<dyn std::error::Error>> {
    let symbol = sqlx::query_as!(Symbol, "SELECT * FROM symbols WHERE code = $1", code)
        .fetch_one(db_pool)
        .await?;
    Ok(symbol)
}

pub async fn insert_symbol(
    db_pool: &PgPool,
    symbol: InsertableSymbol,
) -> Result<(), Box<dyn std::error::Error>> {
    let res = sqlx::query!(
        r#"INSERT INTO symbols (code, name) VALUES ($1, $2) ON CONFLICT (code) DO UPDATE SET name = EXCLUDED.name"#,
        symbol.code,
        symbol.name
    )
    .execute(db_pool)
    .await;
    match res {
        Ok(_) => {}
        Err(e) => eprintln!("Error inserting symbol: {:?} - {:?}", &symbol, e),
    }
    // println!("Inserting symbol: {:?}", &symbol);
    Ok(())
}

pub async fn insert_symbols(
    db_pool: &PgPool,
    symbols: Vec<InsertableSymbol>,
) -> Result<(), Box<dyn std::error::Error>> {
    for symbol in symbols {
        let _ = insert_symbol(db_pool, symbol).await;
    }
    Ok(())
}
