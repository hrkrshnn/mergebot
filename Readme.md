## A bot to deploy the merge oracle in the merge block

[Context](https://twitter.com/alexberegszaszi/status/1570211290555232256)

## Running

```
forge build
cargo run --release
```

Set the environment variables:

1. `MERGEBOT_PRIVKEY`: private key for the account that sends the transaction.
2. `ETH_RPC_URL`: HTTP RPC.
3. `ETH_WSS_URL`: web socket endpoint.

## Hall Of Fame

https://etherscan.io/tx/0xf25815081739f4fff71b857c2007519e9d5b742819a0209dfcf82fed66555d50

`0xd6a6f0d7f08c2d31455a210546f85ddff1d9030a` is the `MergeOracle`.

```
cast call 0xd6a6f0d7f08c2d31455a210546f85ddff1d9030a "mergeTimestamp()"
0x000000000000000000000000000000000000000000000000000000006322c973

cast call 0xd6a6f0d7f08c2d31455a210546f85ddff1d9030a "mergeBlock()"
0x0000000000000000000000000000000000000000000000000000000000ed14f2
```
