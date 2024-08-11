use actix_web::{web, App, HttpServer, Responder};
// use serde::{Deserialize, Serialize};
use app_attest_core::types::AppAttestationRequest;
use risc0_zkvm::{ExecutorEnv, Prover};
use guest_crate::METHOD_ID; //TODO: wadiz?


async fn generate_proof_post(input: web::Json<AppAttestationRequest>) -> impl Responder {
    //Run the proof generation code in the guest code

    // Set up the executor environment
    let env = ExecutorEnv::builder()
        .add_input(&input)
        .build()
        .unwrap();

    // Create a prover instance
    let prover = Prover::new(&METHOD_ID, env).unwrap();

    // Run the guest code
    let receipt = prover.run().unwrap();

    // Verify the receipt
    receipt.verify(METHOD_ID).unwrap();

    // Extract and process the journal
    let journal = receipt.journal;
    let result = process_journal(journal);

    // Return the result as a JSON response
    web::Json(result)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/trigger-guest", web::post().to(trigger_guest_code))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}