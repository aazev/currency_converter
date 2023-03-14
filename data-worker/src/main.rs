use std::{env, str::FromStr};

use chrono::{NaiveDate, NaiveDateTime};
use common::{
    ampq_requests::{RabbitMQRequest, RabbitMQRequestType},
    http::{
        requests::FluctuationsRequest,
        responses::{FluctuationsApiResponse, QuotationsApiResponse, SymbolsApiResponse},
    },
};
use database::models::{
    quotations::{insert_quotations, InsertableQuotation},
    symbols::{insert_symbols, retrieve_symbol_by_code, InsertableSymbol},
};
use deadpool::managed::Object;
use deadpool_lapin::{Manager, Pool};
use dotenv::dotenv;
use futures_lite::stream::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicQosOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    ConnectionProperties,
};
use sqlx::{types::BigDecimal, PgPool};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let worker_prefetch_count = std::env::var("WORKER_PREFETCH_COUNT")
        .unwrap_or_else(|_| num_cpus::get().to_string())
        .parse::<u16>()
        .unwrap();
    let ampq_addr =
        std::env::var("AMPQ_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let manager: Manager = Manager::new(&ampq_addr, ConnectionProperties::default());
    let pool: Pool = deadpool::managed::Pool::builder(manager)
        .max_size(num_cpus::get())
        .build()
        .expect("Failed to create ampq pool");

    let rmq_conn: Object<Manager> = pool
        .get()
        .await
        .map_err(|e| {
            eprintln!("Failed to get connection from pool: {}", e);
            e
        })
        .unwrap();
    let rabbit_mq_channel = rmq_conn.create_channel().await.unwrap();
    let _ = rabbit_mq_channel
        .basic_qos(
            worker_prefetch_count,
            BasicQosOptions {
                global: false,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    println!(
        "Starting RabbitMQ alarm consumer with {} prefetch at {}",
        &worker_prefetch_count, &ampq_addr
    );

    let _r = init_rmq_listen(&rabbit_mq_channel).await;
    println!("RabbitMQ alarm consumer stopped");
}

async fn init_rmq_listen<'a>(channel: &'a lapin::Channel) -> lapin::Result<()> {
    let db_pool = database::pool::connect().await.unwrap();

    let queue_name = std::env::var("RMQ_QUEUE_NAME").unwrap_or_else(|_| "currency-fetcher".into());
    let consumer_prefix = "oxidated-currency-fetcher";
    let consumer_id = uuid::Uuid::new_v4().to_string();
    let consumer_name = format!("{}-{}", consumer_prefix, consumer_id);

    channel
        .queue_declare(
            &queue_name,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            &queue_name,
            &consumer_name,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("RabbitMQ worker connected");
    println!(" [*] Waiting for messages. To exit press CTRL+C");

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            match serde_json::from_slice::<RabbitMQRequest>(&delivery.data) {
                Ok(request) => {
                    let pool_copy = db_pool.clone();
                    let request_copy = request.clone();
                    tokio::spawn(async move {
                        let _ = handle_rabbitmq_request(pool_copy, request_copy).await;
                        let _ = delivery
                            .ack(BasicAckOptions {
                                multiple: false,
                                ..Default::default()
                            })
                            .await;
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                    let _ = delivery
                        .nack(BasicNackOptions {
                            multiple: false,
                            requeue: false,
                            ..Default::default()
                        })
                        .await;
                }
            }
        }
    }
    Ok(())
}

async fn handle_rabbitmq_request(
    db_pool: PgPool,
    request: RabbitMQRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    match &request.request_type {
        // Example rabbit request:
        /*
        {
            "date_query": "2023-03-13T12:00:00",
            "date_start": "2022-01-01",
            "date_end": "2022-12-31",
            "base_symbol":"EUR",
            "request_type":"Fluctuations"
        }
        */
        RabbitMQRequestType::Fluctuations => {
            let api_request = FluctuationsRequest {
                base: request.base_symbol.clone().unwrap(),
                start_date: request.date_start.clone().unwrap().to_string(),
                end_date: request.date_end.clone().unwrap().to_string(),
            };
            let request_uri = std::fmt::format(format_args!(
                "https://api.apilayer.com/exchangerates_data/fluctuation?base={}&start_date={}&end_date={}",
                &api_request.base, &api_request.start_date, &api_request.end_date,
            ));
            let client = reqwest::Client::new();
            println!("Received request: {:?}", &request);
            println!("Api Request: {:?}", &api_request);
            println!("Requesting {}", &request_uri);
            let response = client
                .get(&request_uri)
                .header("apikey", &api_key)
                .send()
                .await?;
            let response_object = response.json::<FluctuationsApiResponse>().await?;

            let mut quotations: Vec<InsertableQuotation> = Vec::new();
            for (symbol_code, rates) in response_object.rates {
                let base_symbol =
                    match retrieve_symbol_by_code(&response_object.base.to_uppercase(), &db_pool)
                        .await
                    {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Base Symbol Error: {}", e);
                            continue;
                        }
                    };
                let symbol =
                    match retrieve_symbol_by_code(&symbol_code.to_uppercase(), &db_pool).await {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Symbol Code Error: {}", e);
                            continue;
                        }
                    };
                match (base_symbol.id, symbol.id) {
                    (base_symbol_id, symbol_id) => {
                        let start_rate = rates.get("start_rate").unwrap();
                        let end_rate = rates.get("end_rate").unwrap();
                        quotations.push(InsertableQuotation {
                            base_symbol_id: base_symbol_id,
                            symbol_id: symbol_id,
                            date: request.date_query.clone(),
                            open: BigDecimal::try_from(start_rate.clone()).unwrap(),
                            close: BigDecimal::try_from(end_rate.clone()).unwrap(),
                        });
                    }
                }
            }
            println!("Inserting {} quotations", &quotations.len());
            let _ = insert_quotations(quotations, &db_pool).await?;
            println!("Inserted quotations");
        }
        RabbitMQRequestType::Quotations => {
            let api_request = FluctuationsRequest {
                base: request.base_symbol.clone().unwrap(),
                start_date: request.date_start.clone().unwrap().to_string(),
                end_date: request.date_end.clone().unwrap().to_string(),
            };
            let request_uri = std::fmt::format(format_args!(
                "https://api.apilayer.com/exchangerates_data/timeseries?base={}&start_date={}&end_date={}",
                &api_request.base, &api_request.start_date, &api_request.end_date,
            ));
            let client = reqwest::Client::new();
            println!("Received request: {:?}", &request);
            println!("Api Request: {:?}", &api_request);
            println!("Requesting {}", &request_uri);
            let response = client
                .get(&request_uri)
                .header("apikey", &api_key)
                .send()
                .await?;
            let response_object = response.json::<QuotationsApiResponse>().await?;

            let mut quotations: Vec<InsertableQuotation> = Vec::new();
            for (date, rates) in response_object.rates {
                for (symbol, rate) in rates {
                    let base_symbol = match retrieve_symbol_by_code(
                        &response_object.base.to_uppercase(),
                        &db_pool,
                    )
                    .await
                    {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Base Symbol Error: {}", e);
                            continue;
                        }
                    };
                    let symbol =
                        match retrieve_symbol_by_code(&symbol.to_uppercase(), &db_pool).await {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("Symbol Code Error: {}", e);
                                continue;
                            }
                        };
                    //get hour from request date_query but use date from api response
                    let time = request.date_query.time();
                    match (base_symbol.id, symbol.id) {
                        (base_symbol_id, symbol_id) => quotations.push(InsertableQuotation {
                            base_symbol_id: base_symbol_id,
                            symbol_id: symbol_id,
                            date: NaiveDateTime::new(
                                NaiveDate::from_str(date.as_str()).unwrap(),
                                time,
                            ),
                            open: BigDecimal::try_from(rate.clone()).unwrap(),
                            close: BigDecimal::try_from(rate.clone()).unwrap(),
                        }),
                    }
                }
            }
            println!("Inserting {} quotations", &quotations.len());
            let _ = insert_quotations(quotations, &db_pool).await?;
            println!("Inserted quotations");
        }
        RabbitMQRequestType::Symbols => {
            //{"date_query":"2023-03-07T19:30:00", "request_type":"Symbols"}
            let request_uri = "https://api.apilayer.com/exchangerates_data/symbols".to_string();
            let client = reqwest::Client::new();
            println!("Received request: {:?}", &request);
            println!("Requesting {}", &request_uri);
            let response = client
                .get(&request_uri)
                .header("apikey", &api_key)
                .send()
                .await?;
            let response_object = response.json::<SymbolsApiResponse>().await?;

            let mut symbols = Vec::new();
            for (key, value) in response_object.symbols {
                symbols.push(InsertableSymbol {
                    code: key,
                    name: value,
                });
            }
            println!("Inserting {} symbols", &symbols.len());
            let _ = insert_symbols(&db_pool, symbols).await?;
            println!("Inserted symbols");
        }
    }
    Ok(())
}
