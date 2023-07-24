use std::collections::HashMap;
use std::net::SocketAddr;

use axum::routing::get;
use axum::{Json, Router};

use axum::{extract::Query, http::HeaderMap, http::StatusCode};
use serde_json::{json, Value};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// .http file name
    #[structopt(long, short, default_value = "3031")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let opt = Opt::from_args();

    // Build App and routes
    let app = Router::new().route("/", get(httpbin).post(httpbin));

    // Bind to port and start server
    let addr = SocketAddr::from(([127, 0, 0, 1], opt.port));

    println!("\nServing at: {}\n", format!("http://{}", addr.to_string()));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub(crate) async fn httpbin(
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: String,
) -> Result<Json<Value>, StatusCode> {
    let mut header_map = serde_json::value::Map::new();

    for (key, val) in headers {
        if let Some(key) = key {
            header_map.insert(key.to_string(), val.to_str().unwrap_or_default().into());
        }
    }

    let json = json!({
        "params": params,
        "headers": header_map,
        "body": body,
    });

    Ok(Json(json))
}
