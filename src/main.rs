use std::{collections::HashSet, io::BufRead, sync::Arc};

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    addr: std::net::SocketAddr,

    #[arg(short, long)]
    list: std::path::PathBuf,
}

struct AppState {
    list: HashSet<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let list = load_list(&args.list);
    println!("loaded list: {}", list.len());
    let state = Arc::new(AppState { list });
    let app = Router::new().route("/", post(verify)).with_state(state);
    let listener = tokio::net::TcpListener::bind(args.addr).await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn verify(
    State(state): State<Arc<AppState>>,
    body: axum::body::Bytes,
) -> Result<Json<DERPAdmitClientResponse>, StatusCode> {
    let mut deserializer = serde_json::Deserializer::from_slice(&body);
    let req = DERPAdmitClientRequest::deserialize(&mut deserializer)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let allow = state.list.contains(&req.NodePublic);
    println!("{}, {}, {}", req.Source, req.NodePublic, allow);
    return Ok(Json(DERPAdmitClientResponse { Allow: allow }));
}

fn load_list(path: &std::path::Path) -> HashSet<String> {
    let mut list = HashSet::new();
    let f = std::fs::File::open(path).unwrap();
    let mut r = std::io::BufReader::new(f);
    let mut line = String::new();
    loop {
        line.clear();
        let n = r.read_line(&mut line).unwrap();
        if n == 0 {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        list.insert(line.to_string());
    }
    list
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct DERPAdmitClientRequest {
    NodePublic: String,
    Source: std::net::IpAddr,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct DERPAdmitClientResponse {
    Allow: bool,
}
