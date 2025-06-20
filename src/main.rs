use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct DecodeParams {
    /// if specified, the visualizer json will be returned in the field `visualizer_json`
    pub visualizer_json: Option<String>,
    /// if specified, the standalone html visualizer will be returned in the field `html`
    pub html: Option<String>,
}

#[get("/decode")]
pub async fn decode(query: web::Query<DecodeParams>) -> impl Responder {
    let mut result = String::new();

    // Combine the parameters into a single string
    let with_html = query.html.is_some();
    println!("with_html: {}", with_html);

    // If no parameters provided, return a default message
    if result.is_empty() {
        result = "No parameters provided".to_string();
    }

    result
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeInfo {
    pub name: String,
    pub description: String,
}

lazy_static! {
    static ref CODE_INFO: HashMap<String, CodeInfo> = {
        let mut map = HashMap::new();
        map.insert(
            "rsc-d-5".to_string(),
            CodeInfo {
                name: "rsc-d-5".to_string(),
                description: "rsc-d-5".to_string(),
            },
        );
        map.insert(
            "color-d-5".to_string(),
            CodeInfo {
                name: "color-d-5".to_string(),
                description: "color-d-5".to_string(),
            },
        );
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

    HttpServer::new(|| App::new().service(index).service(decode))
        .bind((args.ip, args.port))?
        .run()
        .await
}
