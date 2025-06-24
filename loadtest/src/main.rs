use goose::prelude::*;
use goose_eggs::{Validate, validate_and_load_static_assets};
use lazy_static::lazy_static;
use mwpf_web::codes::*;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Arc;

// const NUM_SAMPLES_PER_CODE: usize = 1;
const NUM_SAMPLES_PER_CODE: usize = 100;

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
    static ref TEST_URLS: Vec<String> = {
        let mut urls = vec![];
        for code in CODES.iter() {
            for _ in 0..NUM_SAMPLES_PER_CODE {
                let syndrome = generate_random_syndrome(code);
                if syndrome.is_empty() {
                    continue;  // do not create this transaction
                }
                urls.push(format!(
                    "/api/decode?code_id={}&syndrome={}",
                    code.client_info.id, syndrome
                ));
            }
        }
        urls
    };
}

fn generate_random_syndrome(code: &ServerCodeInfo) -> String {
    let mut syndrome: HashSet<usize> = HashSet::new();
    for actions in code.client_info.data_qubit_actions.iter() {
        let mut error_types = actions.keys().cloned().collect::<Vec<String>>();
        error_types.push("I".to_string());
        let error_type = &error_types[rand::rng().random_range(0..error_types.len())];
        if error_type != "I" {
            let error_syndrome: HashSet<usize> = actions[error_type].iter().cloned().collect();
            syndrome = syndrome
                .symmetric_difference(&error_syndrome)
                .cloned()
                .collect();
        }
    }
    println!("syndrome: {:?}", syndrome);
    syndrome
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    let mut scenario = scenario!("DecodeTransactions");

    for url in TEST_URLS.iter() {
        let closure: TransactionFunction = Arc::new(move |user| {
            let url = url.clone();
            Box::pin(async move {
                let goose = user.get(&url).await?;
                let validate = &Validate::builder().status(200).build();
                validate_and_load_static_assets(user, goose, &validate).await?;
                Ok(())
            })
        });
        let transaction = Transaction::new(closure);
        let new_scenario = scenario.register_transaction(transaction);
        scenario = new_scenario;
    }

    GooseAttack::initialize()?
        .register_scenario(scenario)
        .execute()
        .await?;

    Ok(())
}
