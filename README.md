# devtool

> :warning: This repo is experimental and shouldn't be relied upon!

Some commands that could be helpful for developing or testing Anoma, but that may not make sense to include in `anomac` (currently). For example, submitting a protocol transaction signed with an arbitrary key.

Run `cargo run --help` (or `devtool --help` after having run `make install`) to see all possible commands.

## Example usages

### Submitting a no-op EthereumBridgeUpdate protocol transaction

```shell
cargo run -- write-protocol-tx \
    --code examples/tx_no_op.wasm \
    --key examples/borsh_serialized_secret_key.txt \
    --out protocol_tx.bin
cargo run -- print-tx protocol_tx.bin
# assuming there is an Anoma ledger node running on 127.0.0.1:26657
cargo run -- submit-tx protocol_tx.bin
```