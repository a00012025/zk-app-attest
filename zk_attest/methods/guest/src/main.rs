use app_attest_core::AppAttestationRequest;
use risc0_zkvm::guest::env;

fn main() {
    let request: AppAttestationRequest = env::read();

    // TODO: do something with the input

    // write public output to the journal
    let value = request.value;
    env::commit(&value);
}
