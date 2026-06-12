# 4626 Solana LZ OFT — verifiable source

Minimal source snapshot for the deployed LayerZero V2 OFT program that powers the
4626 Base ↔ Solana share mesh (`■` share tokens).

| | |
|---|---|
| Program ID | [`6ste36Y7fcbzJXkVQj3ApEqYb3wFZsZX63gT6wymhy3s`](https://explorer.solana.com/address/6ste36Y7fcbzJXkVQj3ApEqYb3wFZsZX63gT6wymhy3s/verified-build) |
| Network | Solana mainnet-beta |
| Verified build status | [verify.osec.io](https://verify.osec.io/status/6ste36Y7fcbzJXkVQj3ApEqYb3wFZsZX63gT6wymhy3s) |
| Expected program hash | `ecd321cd2b368afd95917d6d84c30c7bb0fb82cbd507206b328bcf5db4b458a6` |
| Framework | Anchor 0.31.1, Rust 1.84.1, platform-tools v1.51 |
| Upstream | [LayerZero-Labs/LayerZero-v2](https://github.com/LayerZero-Labs/LayerZero-v2) OFT (rev `c09287a`) |

## Reproduce the on-chain hash

Requires Docker and [solana-verify](https://github.com/Ellipsis-Labs/solana-verifiable-build):

```bash
solana-verify verify-from-repo -um \
  --program-id 6ste36Y7fcbzJXkVQj3ApEqYb3wFZsZX63gT6wymhy3s \
  https://github.com/4626fun/oft-solana-6ste-verifiable \
  --library-name oft \
  --cargo-build-sbf-args="--tools-version v1.51"
```

The `--tools-version v1.51` pin is required — the deployed binary was built with
platform-tools v1.51, and other tools versions produce a different (non-matching) hash.

## Layout

- `programs/oft/` — the OFT program (`declare_id!` defaults to the deployed program ID via `OFT_ID` env override)
- `programs/endpoint-mock/` — build-only mock used by the workspace; not deployed
