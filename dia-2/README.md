# Event Pass on Stellar Testnet

## Deployment

- Network: Stellar Testnet
- Stellar CLI: `28.0.0`
- Contract ID: `CAMMJWAC4NOT5Q5BZAYJ7M6HVAYCAAFFNUVV7TAR4CQVWXH7RM5Y527Y`
- WASM hash: `3f52e00dbdf147e532b895ecefb7efe4c61a60f5d13dd3db6b2d3c13a406f5b1`
- WASM path: `target/wasm32v1-none/release/event_pass.wasm`
- Public demo holder: `GAAKZNHT6LVCOHGCBDQQPGQJZEOSOWWKLET2YCW47PLT7SQ7TAOXE762`

The signing identity is stored in the local macOS Keychain. No secret key is stored in this repository.

## Build and Test

From `dia-2/`:

```bash
stellar contract build --manifest-path Cargo.toml --package event-pass --locked
cargo test --workspace
cargo check --workspace
```

The current Soroban SDK and Rust toolchain use the `wasm32v1-none` target. Stellar CLI selects it automatically for this project.

## Testnet Flow

The deployed contract was exercised with the following flow:

1. `buy()` with the demo holder address.
2. Read `holder()` and `state()`, which returned the holder and `Purchased`.
3. `redeem()` with the same holder address.
4. Read `state()`, which returned `Redeemed`.

The successful purchase transaction emitted `PassPurchased`:

- [View buy transaction in Stellar Expert](https://stellar.expert/explorer/testnet/tx/88c2e51c5b424a6f7c0260ae79459df4ddc0acca1e49a27bc461bbdc2ff3270c)

The successful redemption transaction emitted `PassRedeemed`:

- [View redeem transaction in Stellar Expert](https://stellar.expert/explorer/testnet/tx/25439117f5aa0353b67174f75394364ba5d9084abbf0711c3916067eccb0d297)
- [Open contract in Stellar Lab](https://lab.stellar.org/r/testnet/contract/CAMMJWAC4NOT5Q5BZAYJ7M6HVAYCAAFFNUVV7TAR4CQVWXH7RM5Y527Y)

## Reproducible CLI Commands

Create and fund a Testnet-only identity. The `--secure-store` option keeps the key outside the repository:

```bash
stellar keys generate event-pass-demo --secure-store --fund --network testnet
stellar keys public-key event-pass-demo
```

Build and deploy. Use a different contract alias from the identity alias:

```bash
stellar contract build --manifest-path Cargo.toml --package event-pass --locked
stellar contract deploy \
  --wasm target/wasm32v1-none/release/event_pass.wasm \
  --source-account event-pass-demo \
  --network testnet \
  --alias event-pass-contract
```

Invoke `buy()` and inspect its result and event:

```bash
stellar contract invoke \
  --id event-pass-contract \
  --source-account event-pass-demo \
  --network testnet \
  -- buy --buyer <PUBLIC_HOLDER_ADDRESS>
```

Read the holder and state without submitting a transaction:

```bash
stellar contract invoke --id event-pass-contract --source-account event-pass-demo --network testnet --send no -- holder
stellar contract invoke --id event-pass-contract --source-account event-pass-demo --network testnet --send no -- state
```

Invoke `redeem()` and inspect its result and event:

```bash
stellar contract invoke \
  --id event-pass-contract \
  --source-account event-pass-demo \
  --network testnet \
  -- redeem --holder <PUBLIC_HOLDER_ADDRESS>
```

## Explorer Verification

Open the transaction links above in Stellar Expert and confirm:

- The transaction is successful.
- `PassPurchased` appears after `buy()` with topic `pass_purchased`.
- `PassRedeemed` appears after `redeem()` with topic `pass_redeemed`.
- Both events identify the holder address.
- The final `state()` result is `Redeemed`.

Use only Stellar Testnet identities and funds for this demonstration. Never put a secret key, seed phrase, or private credential in this repository.
