# examples

- borsh_serialized_secret_key.txt is a Borsh serialization of a random secret key.

You can get the actual Borsh-serialized protocol key of your validator by looking in `wallet.toml` and taking the part after the `ED25519_SK_PREFIX`.

```rust
[validator_data.keys]
protocol_keypair = "ED25519_SK_PREFIX00361358accc777e5c7b415e3aff290dbff412bea48660ac423d70c1c7f21e3171"
```

- tx_no_op.wasm from https://github.com/anoma/anoma/blob/ce0a07598a2bcb90ac67cab2c25e71e32b68193f/wasm_for_tests/tx_no_op.wasm
