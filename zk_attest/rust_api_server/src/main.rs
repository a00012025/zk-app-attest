use actix_web::{web, App, HttpServer, Responder};
use alloy_primitives::{Address, U256};
use app_attest_core::types::AppAttestationRequest;
use risc0_zkvm::{ExecutorEnv, default_prover};
use serde::{Deserialize, Serialize};
use methods::{ZK_ATTEST_GUEST_ELF, ZK_ATTEST_GUEST_ID}; //TODO: wadiz?

#[derive(Serialize, Deserialize)]
struct Output {
    address: Address,
    value: U256
}

async fn generate_proof_post(input: web::Json<AppAttestationRequest>) -> impl Responder {
    //Run the proof generation code in the guest code

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    // Run the guest code
    let prove_info = prover.prove(env, ZK_ATTEST_GUEST_ELF).unwrap();

    // extract the receipt.
    let receipt = prove_info.receipt;

    // Decode the journal to get the address and value
    let journal_bytes = receipt.journal.bytes.clone();
    let (address, value) = <(Address, U256)>::abi_decode(&journal_bytes, true).unwrap();

    println!("Address: {:?}", address);
    println!("Value: {}", value);

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt.verify(ZK_ATTEST_GUEST_ID).unwrap();
    
    // Return the result as a JSON response
    web::Json(Output { address, value })
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/trigger-host", web::post().to(generate_proof_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}