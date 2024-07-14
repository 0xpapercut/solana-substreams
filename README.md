# raydium-substream
Stream Raydium events with [substreams](https://substreams.streamingfast.io).

## Usage
1. Setup the environment variable `STREAMINGFAST_KEY` with an [API key](https://app.streamingfast.io/keys).
2. Run `. ./token.sh`
3. Start streaming with `make stream START=<slot>`. You can verify the most recent slot on the [Solana Explorer](https://explorer.solana.com).

Suported events include swap, initialize, deposit and withdraw. For more information, refer to the [protobuf specification](proto/raydium.proto).

For storing these events in a database, a [schema](schema.sql) is provided along with a corresponding `table_changes` function. [Find here an example of usage](https://github.com/0xpapercut/substream-sinks/tree/main/solana-clickhouse).
