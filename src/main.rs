pub mod codes;

use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;

use codes::*;

#[derive(Debug, Deserialize)]
pub struct DecodeParams {
    /// if specified, the visualizer json will be returned in the field `visualizer_json`
    pub visualizer_json: Option<String>,
    /// if specified, the standalone html visualizer will be returned in the field `html`
    pub html: Option<String>,
}

#[get("/api/decode")]
pub async fn decode(query: web::Query<DecodeParams>) -> impl Responder {
    let mut result = String::new();

    // Combine the parameters into a single string
    let with_html = query.html.is_some();

    // If no parameters provided, return a default message
    if result.is_empty() {
        result = "No parameters provided".to_string();
    }

    result
}

#[get("/api/codes")]
pub async fn get_codes() -> impl Responder {
    let codes: Vec<ClientCodeInfo> = CODES.iter().map(|code| code.client_info.clone()).collect();
    serde_json::to_string(&codes).unwrap()
}

lazy_static! {
    static ref CODES: Vec<ServerCodeInfo> = {
        vec![
            ServerCodeInfo::from(&RotatedSurfaceCode::new(3, NoiseType::Depolarize)),
            ServerCodeInfo::from(&RotatedSurfaceCode::new(5, NoiseType::Depolarize)),
            ServerCodeInfo::from(&RotatedSurfaceCode::new(3, NoiseType::BitFlip)),
            ServerCodeInfo::from(&RotatedSurfaceCode::new(5, NoiseType::BitFlip)),
            ServerCodeInfo::from(&RotatedSurfaceCode::new(3, NoiseType::OnlyY)),
            ServerCodeInfo::from(&RotatedSurfaceCode::new(5, NoiseType::OnlyY)),
            // ServerCodeInfo::from(&ColorCode::new(3)),
            // ServerCodeInfo::from(&ColorCode::new(5)),
        ]
    };
    static ref CODES_MAP: HashMap<String, ServerCodeInfo> = {
        let mut map = HashMap::new();
        for code in CODES.iter() {
            map.insert(code.client_info.name.clone(), code.clone());
        }
        map
    };
}

#[get("/")]
pub async fn index() -> impl Responder {
    "MWPF Backend is running! Use /decode with query parameters."
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP address to bind to
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,
    /// Port to bind to
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = Args::parse();

    log::info!(
        "Starting MWPF Backend server on {}:{}...",
        args.ip,
        args.port
    );

    HttpServer::new(|| App::new().service(index).service(decode).service(get_codes))
        .bind((args.ip, args.port))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_info() {
        // cargo test test_code_info -- --nocapture
    }
}
