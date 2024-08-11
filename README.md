# zk-app-attest

## Build zkEVM and Contract

```bash
cd zk_attest
cargo build
cd ../contract
forge build
```

## Build and Run App

```bash
cd app
flutter run ios --debug -t lib/main.dart
```

## Deployment

```bash
cd contract
forge script script/Deploy.s.sol --rpc-url "https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}" --broadcast
forge verify-contract --constructor-args 0x000000000000000000000000925d8331ddc0a1F0d96E68CF073DFE1d92b69187 --chain-id 11155111 0x0bAd2B70c89a5fe1EC6C546C22831Cc7ca22bfe1 contracts/ZkAlcoholAttest.sol:ZkAlcoholAttest
```

## Submit Proof

1. Submit proof to Sepolia and mint an NFT:

    ```bash
    cargo run --bin publisher -- \
        --chain-id=11155111 \
        --rpc-url="https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}" \
        --contract=0x0bAd2B70c89a5fe1EC6C546C22831Cc7ca222bfe1
    ```

2. Submit proof to Aligned

    ```bash
    aligned submit \
        --proving_system Risc0 \
        --proof risc_zero_zk_attest.proof \
            --vm_program zk_attest_id.bin \
            --public_input risc_zero_zk_attest.pub \
        --proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
        --rpc 'https://ethereum-holesky-rpc.publicnode.com' \
        --batcher_addr '0x815aeCA64a974297942D2Bbf034ABEe22a38A003'
    ```
