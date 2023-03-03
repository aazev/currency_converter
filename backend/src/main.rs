mod requests;
mod responses;
mod types;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router, Server};
use clap::Parser;
use hyperlocal::UnixServerExt;
use rand::prelude::*;
use rand::rngs::StdRng;
use serde_json::Value;
use std::{env, net::SocketAddr, path};
use types::ServiceMode;

#[derive(Parser)]
struct Opts {
    #[arg(short = 'm', long = "mode", value_enum, default_value = "address")]
    mode: ServiceMode,
}

fn min_moves_to_sort_array(arr: &mut [i32]) -> i32 {
    let mut target: Vec<i32> = arr.iter().cloned().collect();
    target.sort();
    let mut count = 0;

    for i in 0..arr.len() {
        if arr[i] != target[i] {
            let j = arr.iter().position(|x| *x == target[i]).unwrap();
            arr.swap(i, j);
            count += 1;
        }
    }

    count
}

async fn socket_serve() {
    let socket_addr = env::var("SOCKET_ADDR").expect("SOCKET_ADDR must be set.");
    let socket_path = path::Path::new(&socket_addr);
    match socket_path.exists() {
        true => {
            println!("Removing existing socket file.");
            std::fs::remove_file(socket_path).expect("Failed to remove socket file.");
        }
        false => println!("No existing socket file found."),
    }

    let app = Router::new().route("/", get(home)).fallback(deal_with_it);

    println!("Starting server on socket: {}", socket_addr);

    Server::bind_unix(socket_path)
        .expect("Failed to bind to socket.")
        .serve(app.into_make_service())
        .await
        .expect("Server error.");
}

async fn address_serve() {
    let address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set.");
    let server_address: SocketAddr = address
        .parse::<SocketAddr>()
        .expect("Failed to parse server address.");

    let app = Router::new().route("/", get(home)).fallback(deal_with_it);

    println!("Starting server on address: {}", &address);

    Server::bind(&server_address)
        .serve(app.into_make_service())
        .await
        .expect("Server error.");
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // let opts: Opts = Opts::parse();

    // match opts.mode {
    //     ServiceMode::Socket => {
    //         socket_serve().await;
    //     }
    //     ServiceMode::Address => {
    //         address_serve().await;
    //     }
    // }
    let file = std::fs::File::open("./json/quotations.json").unwrap();
    let data: responses::QuotationResponse = serde_json::from_reader(file).unwrap();

    println!("{:?}", data);
}

async fn home() -> Result<Json<Value>, (StatusCode, String)> {
    Ok(Json(serde_json::json!({"message": "Hello, World!"})))
}

//returns a 404 status json
async fn deal_with_it() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json("Not Found"))
}
