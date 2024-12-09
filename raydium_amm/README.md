# raydium-amm-substream
Stream Raydium events with [substreams](https://substreams.streamingfast.io).

## Usage
```bash
substreams gui raydium-amm-events
```
If you see no output, please check that you have set a starting block, e.g. `substreams gui raydium-amm-events -s 300000000`.

Suported events include swap, initialize, deposit and withdraw. For more information, refer to the [protobuf specification](proto/raydium.proto).
