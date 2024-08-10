pub mod assertion;
pub mod attestation;
pub mod certificate;
pub mod root;
pub mod types;
pub mod utils;

/*
 * First decode the raw attestation and key_id objects. Then validate.
 */
pub fn validate_raw_attestation(
    raw_attestation: &str,
    raw_key_id: &str,
    challenge: &str,
    app_id: &str,
    production: bool,
    leaf_cert_only: bool,
) -> bool {
    let attestation = utils::decode_attestation(raw_attestation.to_string()).unwrap();
    let key_id = utils::decode_base64_to_bytes(&raw_key_id.to_string());

    validate_decoded_attestation(
        attestation,
        challenge.to_string(),
        key_id,
        app_id.to_string(),
        production,
        leaf_cert_only,
    )
}

/*
 * Validate a decoded attestation object.
 */
pub fn validate_decoded_attestation(
    attestation: types::AttestationObject,
    challenge: String,
    key_id: Vec<u8>,
    app_id: String,
    production: bool,
    leaf_cert_only: bool,
) -> bool {
    attestation::validate_attestation(
        attestation,
        challenge,
        key_id,
        app_id,
        production,
        leaf_cert_only,
    )
}

/*
 * Decode a raw assertion and client data object. Then validate.
 */
pub fn validate_raw_assertion(
    raw_assertion: &str,
    raw_client_data: &str,
    public_key_uncompressed_hex: &str,
    client_app_id: &str,
    stored_challenge: &str,
    prev_counter: u32,
) -> bool {
    let assertion = utils::decode_assertion(raw_assertion.to_string()).unwrap();
    let client_data = utils::decode_base64_to_bytes(&raw_client_data.to_string());

    validate_decoded_assertion(
        assertion,
        client_data,
        public_key_uncompressed_hex.to_string(),
        client_app_id.to_string(),
        stored_challenge.to_string(),
        prev_counter,
    )
}

/*
 * Validate a decoded assertion object.
 */
pub fn validate_decoded_assertion(
    assertion: types::AssertionObject,
    client_data: Vec<u8>,
    public_key_uncompressed_hex: String,
    client_app_id: String,
    stored_challenge: String,
    prev_counter: u32,
) -> bool {
    assertion::validate_assertion(
        assertion,
        client_data,
        public_key_uncompressed_hex,
        client_app_id,
        stored_challenge,
        prev_counter,
    )
}

// Decode base64 string into bytes.
pub fn decode_base64_to_bytes(encoded: String) -> Vec<u8> {
    let decoded = utils::decode_base64_to_bytes(&encoded);
    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example usage to validate raw attestation object.
    #[test]
    fn test_validate_attestation() {
        // Valid attestation object.
        let raw_attestation: &str = "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";
        let challenge: &str = "35da2fc3-8616-4e3b-8cef-a526792e50fb:1723314948:231";
        let raw_key_id: &str = "b0Tlq5x/ObfPbUZ5odcZPPvCrU/daLYWaM9pHVcSZi4=";
        let app_id: &str = "37588RFBBA.com.zk-app-attest.dev";

        // Production is set to false.
        let result =
            validate_raw_attestation(raw_attestation, raw_key_id, challenge, app_id, false, false);

        assert_eq!(result, true);
    }

    // Example of validating a raw assertion object.
    #[test]
    fn test_validate_assertion() {
        // Valid assertion object.
        let encoded_assertion: &str = "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";
        let client_data_encoded: &str = "eyJjaGFsbGVuZ2UiOiJhc3NlcnRpb24tdGVzdCJ9";

        let stored_challenge: &str = "assertion-test";
        let client_id: &str = "example";
        let prev_counter: u32 = 0;
        let public_key_uncompressed_hex: &str = "0437c404fa2bbf8fbcf4ee7080573d5fa80c4f6cc3a22f7db43af92c394e7cd1c880c95ab422972625e8e673af1bda2b096654e9b602895601f925bb5941c53082";

        let result = validate_raw_assertion(
            encoded_assertion,
            client_data_encoded,
            public_key_uncompressed_hex,
            client_id,
            stored_challenge,
            prev_counter,
        );

        assert_eq!(result, true);
    }
}