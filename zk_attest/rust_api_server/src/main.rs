use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
// use crate::simple_math::calculate_max;


#[derive(Deserialize)]
struct InputData { // Input comign from device side
    time: String,
    voltage_array: [f32; 10],
    run_id: String
}AppAttestationRequest -> input

#[derive(Serialize)]
struct OutputData { //Output to put into zkVM guest code
    time: String,
    drunk_or_not: bool,
    run_id: String
} -> (address, value).abi_encode

pub fn calculate_max(numbers: &[f32]) -> f32 {
    *numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

async fn handle_post(input: web::Json<SensorInputData>) -> impl Responder {
    let output = OutputData {
        time: input.time.clone(),
        drunk_or_not: if calculate_max(&input.voltage_array),
        run_id: input.run_id.clone()
    };
    
    web::Json(output)
}

async fn generate_proof_post(input: web::Json<InputData>) -> impl Responder {
    let proof = generate_proof(&input.run_id);
    web::Json(proof)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/api")
                .route(web::post().to(handle_post))
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}