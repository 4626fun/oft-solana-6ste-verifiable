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

Previous verified hash (pre `admin_set_token_mint_authority`): `ecd321cd2b368afd95917d6d84c30c7bb0fb82cbd507206b328bcf5db4b458a6`

Pending upgrade hash (local `cargo-build-sbf --tools-version v1.51` sha256 of `oft.so`): `afc17fa3ab734466458228aae14ee446e9c42d0e851da2017ca16f2068c451b9`

Re-run solana-verify after mainnet upgrade and replace this section with the on-chain executable hash.
