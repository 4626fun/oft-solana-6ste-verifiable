# Verified build

This snapshot is the source for the deployed Solana program
`6ste36Y7fcbzJXkVQj3ApEqYb3wFZsZX63gT6wymhy3s` (LayerZero V2 OFT, Base ↔ Solana share mesh).

Reproduce the on-chain hash with [solana-verify](https://github.com/Ellipsis-Labs/solana-verifiable-build):

```bash
solana-verify verify-from-repo -um \
  --program-id 6ste36Y7fcbzJXkVQj3ApEqYb3wFZsZX63gT6wymhy3s \
  https://github.com/4626fun/oft-solana-6ste-verifiable \
  --library-name oft \
  -- --tools-version v1.51
```

## Deploy history

- Pre `admin_set_token_mint_authority` verified hash: `ecd321cd2b368afd95917d6d84c30c7bb0fb82cbd507206b328bcf5db4b458a6`
- Upgrade with `admin_set_token_mint_authority` (mainnet 2026-07-28):
  - local `cargo-build-sbf --tools-version v1.51` sha256(`oft.so`): `afc17fa3ab734466458228aae14ee446e9c42d0e851da2017ca16f2068c451b9`
  - upgrade tx: `4MuRDBe7PqukLmiB3znxfCpbzywVeCGQJS8xvXEU8fd6bX4mtpQq8VRy9oLj9vyxAJHdH3qKSnZAKhYdihh4Bx1E`
  - Re-run `solana-verify` and replace with the on-chain executable hash when convenient.
