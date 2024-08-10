use x509_cert::Certificate;
use x509_verify::{der::DecodePem, Error, VerifyingKey};

use crate::utils::b64_to_pem;
use std::collections::HashSet;

// Validate certificate chain.
pub fn validate_certificate_path(cert_path: Vec<String>, leaf_cert_only: bool) -> bool {
    if cert_path.len() != cert_path.iter().collect::<HashSet<_>>().len() {
        panic!("Duplicate certificates in certificate path.");
    }

    let num_runs = if leaf_cert_only { 1 } else { cert_path.len() };

    for i in 0..num_runs {
        // Decode subject certificate.
        let subject_b64_data = cert_path[i].as_str();
        let subject_pem = b64_to_pem(&subject_b64_data);

        let subject_cert = Certificate::from_pem(&subject_pem).unwrap();
        // Decode issuer certificate.
        let issuer_b64_data: &str;
        if i + 1 >= cert_path.len() {
            // If this is the last certificate in the path, then the issuer is the root certificate (self-signed).
            issuer_b64_data = subject_b64_data;
        } else {
            issuer_b64_data = cert_path[i + 1].as_str();
        }

        let issuer_pem = b64_to_pem(&issuer_b64_data);
        let issuer_cert = Certificate::from_pem(&issuer_pem).unwrap();

        let key = VerifyingKey::try_from(&issuer_cert).unwrap();

        // verify the signature
        match key.verify(&subject_cert) {
            Ok(_) => {}
            Err(Error::Verification) => {
                println!("Verification error");
            }
            Err(e) => {
                println!("Verification error {:?}", e);
                return false;
            }
        }
    }
    true
}
