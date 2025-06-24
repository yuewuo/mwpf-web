pub mod codes;

use actix_web::{App, HttpRequest, HttpServer, Responder, Result, get, web};
use clap::Parser;
use codes::*;
use lazy_static::lazy_static;
use mwpf::mwpf_solver::{SolverSerialJointSingleHair, SolverTrait};
use mwpf::util::*;
use mwpf::visualize::*;
use num_traits::cast::ToPrimitive;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct DecodeParams {
    /// the id of the code type
    pub code_id: String,
    /// the syndrome in the format of "1,2,3"
    pub syndrome: String,
    /// if specified, the visualizer json will be returned in the field `visualizer_json`
    pub with_json: Option<String>,
    /// if specified, the standalone html visualizer will be returned in the field `html`
    pub with_html: Option<String>,
    /// the maximum number of nodes per cluster
    /// if not specified, the default value is 200
    #[serde(default = "default_cluster_node_limit")]
    pub cluster_node_limit: usize,
}

fn default_cluster_node_limit() -> usize {
    200
}

pub struct DecodeResult {
    pub correction: Vec<(usize, String)>,
    pub weight_range: WeightRange,
    pub visualizer: Option<Visualizer>,
}

pub async fn decode_common(query: &DecodeParams) -> Result<DecodeResult> {
    let code_id = query.code_id.clone();
    let code = CODES_MAP
        .get(&code_id)
        .ok_or(actix_web::error::ErrorBadRequest(
            "Code not found".to_string(),
        ))?;
    let syndrome: Vec<usize> = query
        .syndrome
        .split(',')
        .map(|s| {
            s.parse::<usize>().map_err(|_| {
                actix_web::error::ErrorBadRequest("Invalid syndrome format".to_string())
            })
        })
        .collect::<Result<Vec<usize>, actix_web::Error>>()?;
    for vertex_index in syndrome.iter() {
        if *vertex_index >= code.visualize_positions.len() {
            return Err(actix_web::error::ErrorBadRequest(
                "Invalid syndrome due to vertex index out of range".to_string(),
            ));
        }
    }
    // construct decoder
    let mut visualizer = None;
    if query.with_json.is_some() || query.with_html.is_some() {
        visualizer = Some(
            Visualizer::new(Some(String::new()), code.visualize_positions.clone(), true).unwrap(),
        );
    }
    let mut solver = SolverSerialJointSingleHair::new(
        &Arc::new(code.solver_initializer.clone()),
        json!({"cluster_node_limit": query.cluster_node_limit}),
    );
    let syndrome_pattern = SyndromePattern::new_vertices(syndrome.clone());
    solver.solve_visualizer(syndrome_pattern.clone(), visualizer.as_mut());
    let (subgraph, weight_range) = solver.subgraph_range_visualizer(visualizer.as_mut());

    let correction: Vec<(usize, String)> = subgraph
        .iter()
        .map(|edge_index| code.edge_errors[*edge_index].clone())
        .collect();

    Ok(DecodeResult {
        correction,
        weight_range,
        visualizer,
    })
}

#[get("/api/decode")]
pub async fn decode(req: HttpRequest, query: web::Query<DecodeParams>) -> Result<impl Responder> {
    // log user request
    let remote_ip = req
        .connection_info()
        .realip_remote_addr()
        .map(|ip| ip.to_string());
    log::info!(
        "Decode request from {:?}: code_id={}, syndrome={}, with_json={}, with_html={}, cluster_node_limit={}",
        remote_ip,
        query.code_id,
        query.syndrome,
        query.with_json.is_some(),
        query.with_html.is_some(),
        query.cluster_node_limit,
    );

    // return Err(actix_web::error::ErrorBadRequest("Debug".to_string())); // debug
    // smol::Timer::after(std::time::Duration::from_secs(3)).await; // debug

    let query = query.into_inner();
    let mut decoded = decode_common(&query).await?;

    let mut result: serde_json::Map<String, serde_json::Value> = json!({
        "correction": decoded.correction,
        "lower": decoded.weight_range.lower.to_f64(),
        "upper": decoded.weight_range.upper.to_f64(),
    })
    .as_object()
    .unwrap()
    .clone();
    if query.with_json.is_some() {
        result.insert(
            "json".to_string(),
            decoded.visualizer.as_mut().unwrap().get_visualizer_data(),
        );
    }
    if query.with_html.is_some() {
        result.insert(
            "html".to_string(),
            decoded
                .visualizer
                .as_mut()
                .unwrap()
                .generate_html(json!({}))
                .into(),
        );
    }

    Ok(web::Json(result))
}

#[get("/api/decoding-process")]
pub async fn decoding_process(
    req: HttpRequest,
    query: web::Query<DecodeParams>,
) -> Result<impl Responder> {
    // log user request
    let remote_ip = req
        .connection_info()
        .realip_remote_addr()
        .map(|ip| ip.to_string());
    log::info!(
        "Decode request from {:?}: code_id={}, syndrome={}, with_json={}, with_html={}, cluster_node_limit={}",
        remote_ip,
        query.code_id,
        query.syndrome,
        query.with_json.is_some(),
        query.with_html.is_some(),
        query.cluster_node_limit,
    );

    // return Err(actix_web::error::ErrorBadRequest("Debug".to_string())); // debug
    // smol::Timer::after(std::time::Duration::from_secs(3)).await; // debug

    let mut query = query.into_inner();
    query.with_html = Some("".to_string());
    let mut decoded = decode_common(&query).await?;
    let html = decoded
        .visualizer
        .as_mut()
        .unwrap()
        .generate_html(json!({}));

    Ok(web::Html::new(html))
}

#[get("/api/codes")]
pub async fn get_codes(req: HttpRequest) -> impl Responder {
    // log user request
    let remote_ip = req
        .connection_info()
        .realip_remote_addr()
        .map(|ip| ip.to_string());
    log::info!("Codes request from {:?}", remote_ip,);

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
            ServerCodeInfo::from(&TriangularColorCodeBitFlip::new(3)),
            ServerCodeInfo::from(&TriangularColorCodeBitFlip::new(5)),
        ]
    };
    static ref CODES_MAP: HashMap<String, ServerCodeInfo> = {
        let mut map = HashMap::new();
        for code in CODES.iter() {
            map.insert(code.client_info.id.clone(), code.clone());
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

    log::info!(
        "Available codes: {:?}",
        CODES
            .iter()
            .map(|code| code.client_info.id.clone())
            .collect::<Vec<_>>()
    );

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(decode)
            .service(get_codes)
            .service(decoding_process)
    })
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
