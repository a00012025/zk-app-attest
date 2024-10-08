use alloy_primitives::{Address, U256};
use alloy_sol_types::SolValue;
use app_attest_core::types::AppAttestationRequest;
use bincode;
use methods::{ZK_ATTEST_GUEST_ELF, ZK_ATTEST_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

const PROOF_FILE_PATH: &str = "risc_zero_zk_attest.proof";
const PUB_INPUT_FILE_PATH: &str = "risc_zero_zk_attest.pub";
const ZK_ATTEST_ID_FILE_PATH: &str = "zk_attest_id.bin";

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let challenge_uuid = "35da2fc3-8616-4e3b-8cef-a526792e50fb";
    let challenge_timestamp = "1723314948";
    let value = "231";
    let key_id = "b0Tlq5x/ObfPbUZ5odcZPPvCrU/daLYWaM9pHVcSZi4=";
    let app_id = "37588RFBBA.com.zk-app-attest.dev";
    let raw_attestation: &str = "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";

    // init AppAttestationRequest
    let request = AppAttestationRequest {
        challenge_uuid: challenge_uuid.to_string(),
        challenge_timestamp: challenge_timestamp.to_string(),
        value: value.to_string(),
        key_id: key_id.to_string(),
        app_id: app_id.to_string(),
        raw_attestation: raw_attestation.to_string(),
        address: "0x0901549Bc297BCFf4221d0ECfc0f718932205e33".to_string(),
    };

    let env = ExecutorEnv::builder()
        .write(&request)
        .unwrap()
        .build()
        .unwrap();
    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
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

    // generate files needed by aligned
    let serialized = bincode::serialize(&receipt).unwrap();

    std::fs::write(PROOF_FILE_PATH, serialized).expect("Failed to write proof file");

    std::fs::write(ZK_ATTEST_ID_FILE_PATH, convert(&ZK_ATTEST_GUEST_ID))
        .expect("Failed to write zk_attest_id file");

    std::fs::write(PUB_INPUT_FILE_PATH, receipt.journal.bytes)
        .expect("Failed to write pub_input file");
}

pub fn convert(data: &[u32; 8]) -> [u8; 32] {
    let mut res = [0; 32];
    for i in 0..8 {
        res[4 * i..4 * (i + 1)].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}
