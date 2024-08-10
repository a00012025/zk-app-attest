use alloy_primitives::{Address, U256};
use alloy_sol_types::SolValue;
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
    let result = validate_attestation(attestation, challenge, key_id, request.app_id, false, false);
    assert_eq!(result, true);

    // Parse and pad the address
    let address = Address::parse_checksummed(&request.address, None).unwrap();

    // Parse and pad the value
    let value = U256::from_str_radix(&request.value, 10).unwrap();

    // ABI encode the address and value
    let encoded = (address, value).abi_encode();

    // Commit the ABI encoded data to the journal
    env::commit_slice(encoded.as_slice());
}
