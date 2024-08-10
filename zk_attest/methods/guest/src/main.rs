use app_attest_core::attestation::validate_attestation;
use app_attest_core::types::{AppAttestationRequest, AttestationObject};
use app_attest_core::utils;
use risc0_zkvm::guest::env;

fn main() {
    let request: AppAttestationRequest = env::read();

    let attestation: AttestationObject =
        utils::decode_attestation(request.raw_attestation.to_string()).unwrap();
    let key_id = utils::decode_base64_to_bytes(&request.key_id.to_string());
    let challenge = format!(
        "{}:{}:{}",
        request.challenge_uuid, request.challenge_timestamp, request.value
    );
    validate_attestation(attestation, challenge, key_id, request.app_id, false, false);

    // write public output to the journal
    let value = request.value;
    env::commit(&value);
}
