use super::symbols::Symbol;
use bigdecimal::BigDecimal;
use chrono::{Days, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    pub id: i64,
    pub symbol_id: i64,
    pub base_symbol_id: i64,
    pub date: NaiveDateTime,
    pub open: BigDecimal,
    pub close: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertableQuotation {
    pub symbol_id: i64,
    pub base_symbol_id: i64,
    pub date: NaiveDateTime,
    pub open: BigDecimal,
    pub close: BigDecimal,
}

pub async fn retrieve_quotations(
    symbol: Symbol,
    db_pool: &PgPool,
) -> Result<Vec<Quotation>, Box<dyn std::error::Error>> {
    let mut quotations = Vec::new();
    //get current UTC datetime
    let now = chrono::Utc::now();
    let now = now.checked_sub_days(Days::new(7)).unwrap();
    //get current date
    let today = now.date_naive();

    let rows = sqlx::query_as!(
        Quotation,
        r#"
        SELECT q.*
        FROM quotations q
        JOIN (
            SELECT symbol_id, base_symbol_id, date(MAX(date)) as max_date
            FROM quotations
            GROUP BY symbol_id, base_symbol_id, date(date)
        ) q_max
        ON q.symbol_id = q_max.symbol_id
        AND q.base_symbol_id = q_max.base_symbol_id
        AND date(q.date) = q_max.max_date
        where
            q.symbol_id = $1 AND
            q.date >= $2
        order by symbol_id asc, date DESC;
        "#,
        symbol.id,
        NaiveDateTime::new(today, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    )
    .fetch_all(db_pool)
    .await?;
    for row in rows {
        quotations.push(row);
    }
    Ok(quotations)
}

pub async fn insert_quotation(
    db_pool: &PgPool,
    quotation: InsertableQuotation,
) -> Result<(), Box<dyn std::error::Error>> {
    let _res = sqlx::query!(
        r#"INSERT INTO quotations (symbol_id, base_symbol_id, date, open, close) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (symbol_id, date) DO UPDATE SET open = EXCLUDED.open, close = EXCLUDED.close"#,
        quotation.symbol_id,
        quotation.base_symbol_id,
        quotation.date,
        quotation.open,
        quotation.close
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

pub async fn insert_quotations(
    quotations: Vec<InsertableQuotation>,
    db_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = db_pool.begin().await?;
    for quotation in quotations {
        let res = sqlx::query!(
            r#"INSERT INTO quotations (symbol_id, base_symbol_id, date, open, close) VALUES ($1, $2, $3, $4, $5) ON CONFLICT on CONSTRAINT "quotations_date_symbol_base" DO UPDATE SET open = EXCLUDED.open, close = EXCLUDED.close"#,
            quotation.symbol_id,
            quotation.base_symbol_id,
            quotation.date,
            quotation.open,
            quotation.close
        )
        .execute(&mut transaction)
        .await;
        match res {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Error: {}", e);
                transaction.rollback().await?;
                return Err(Box::new(e));
            }
        }
    }
    transaction.commit().await?;
    Ok(())
}
