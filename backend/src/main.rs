pub mod responses;
pub mod types;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, IntoMakeService},
    Json, Router, Server,
};
use clap::Parser;
use database::{
    models::symbols::{retrieve_all_symbols, retrieve_symbol},
    pool::connect,
};
use hyper::server::conn::AddrIncoming;
use hyperlocal::{SocketIncoming, UnixServerExt};
use responses::symbols::{SymbolResponse, SymbolsResponse};
use serde_json::Value;
use sqlx::PgPool;
use std::{env, net::SocketAddr, path};
use tokio::signal::ctrl_c;
use types::ServiceMode;

#[derive(Parser)]
struct Opts {
    #[arg(short = 'm', long = "mode", value_enum, default_value = "address")]
    mode: ServiceMode,
}

fn socket_serve(rt: Router) -> Server<SocketIncoming, IntoMakeService<Router>> {
    let socket_addr = env::var("SOCKET_ADDR").expect("SOCKET_ADDR must be set.");
    let socket_path = path::Path::new(&socket_addr);
    match socket_path.exists() {
        true => {
            println!("Removing existing socket file.");
            std::fs::remove_file(socket_path).expect("Failed to remove socket file.");
        }
        false => println!("No existing socket file found."),
    }

    println!("Starting server on socket: {}", socket_addr);

    Server::bind_unix(socket_path)
        .expect("Failed to bind to socket.")
        .serve(rt.into_make_service())
}

fn address_serve(rt: Router) -> Server<AddrIncoming, IntoMakeService<Router>> {
    let address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set.");
    let server_address: SocketAddr = address
        .parse::<SocketAddr>()
        .expect("Failed to parse server address.");

    println!("Starting server on address: {}", &address);

    Server::bind(&server_address).serve(rt.into_make_service())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let api = Router::new().route("/", get(home));
    let symbols = Router::new()
        .route("/", get(get_symbols))
        .route("/:id", get(get_symbol));

    let api = api.nest("/symbols", symbols);

    let app = Router::new()
        .nest("/api/v1/", api)
        .fallback(deal_with_it)
        .with_state(connect().await.unwrap());

    let opts: Opts = Opts::parse();

    // let runtime = Runtime::new().unwrap();
    let server_handle = tokio::spawn(async move {
        match opts.mode {
            ServiceMode::Socket => {
                let _ = socket_serve(app).await;
            }
            ServiceMode::Address => {
                let _ = address_serve(app).await;
            }
        };
    });

    ctrl_c().await.unwrap();
    server_handle.abort();

    // let file = std::fs::File::open("./json/quotations.json").unwrap();
    // let data: responses::QuotationResponse = serde_json::from_reader(file).unwrap();

    // println!("{:?}", data);
}

async fn home() -> Result<Json<Value>, (StatusCode, String)> {
    Ok(Json(
        serde_json::json!({"code":200, "message": "Hello, World!"}),
    ))
}

async fn get_symbols(
    State(state): State<PgPool>,
) -> Result<Json<SymbolsResponse>, (StatusCode, String)> {
    let symbols = retrieve_all_symbols(&state).await.unwrap();
    Ok(Json(SymbolsResponse {
        code: 200,
        message: None,
        symbols,
    }))
}

async fn get_symbol(
    Path(id): Path<i64>,
    State(state): State<PgPool>,
) -> Result<Json<SymbolResponse>, (StatusCode, String)> {
    let symbol = retrieve_symbol(id, &state).await.unwrap();
    Ok(Json(SymbolResponse {
        code: 200,
        message: None,
        symbol,
    }))
}

//returns a 404 status json
async fn deal_with_it() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error":404,"message": "Not found"})),
    )
}
