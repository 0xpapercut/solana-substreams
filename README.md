# spl-token-substream
Stream SPL Token Program events with [substreams](https://substreams.streamingfast.io).

## Usage
1. Setup the environment variable `STREAMINGFAST_KEY` with an [API key](https://app.streamingfast.io/keys).
2. Run `. ./token.sh`
3. Start streaming with `make stream START=<slot>`. You can verify the most recent slot on the [Solana Explorer](https://explorer.solana.com).

This substream provides a complete description of SPL Token Program events, as per [spl_token.proto](proto/spl_token.proto).

For storing these events in a database, a [schema](schema.sql) is provided along with a corresponding `table_changes` function. [Find here an example of usage](https://github.com/0xpapercut/substream-sinks/tree/main/solana-clickhouse).
