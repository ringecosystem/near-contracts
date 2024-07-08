# Cross-Contract Hello Contract

## Deploy

```bash
cargo test
## With init
near contract deploy guantong.testnet use-file ./target/wasm32-unknown-unknown/release/cross_contract.wasm with-init-call init json-args '{"mpc_account": "v2.multichain-mpc.testnet"}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-legacy-keychain send
## Without init
near contract deploy guantong.testnet use-file ./target/wasm32-unknown-unknown/release/cross_contract.wasm without-init-call network-config testnet sign-with-legacy-keychain send
```

## Call sign

```bash
near contract call-function as-transaction guantong.testnet request_sign json-args '{"payload": [212,185,105,173,131,20,65,203,72,238,133,58,103,64,139,14,206,76,185,149,205,180,50,237,211,70,250,211,212,11,231,199], "path": "ethereum-1"}' prepaid-gas '300.0 Tgas' attached-deposit '0 NEAR' sign-as guantong.testnet network-config testnet sign-with-legacy-keychain send
```
