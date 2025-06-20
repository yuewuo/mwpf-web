use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

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

#[get("/")]
pub async fn index() -> impl Responder {
    "MWPF Backend is running! Use /decode with query parameters."
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting MWPF Backend server...");

    HttpServer::new(|| App::new().service(index).service(decode))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
