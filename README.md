# raydium-substream
Stream Raydium events with [substreams](https://substreams.streamingfast.io).

## Usage
First, setup your Substreams API token. This can be done using `token.sh`. Setup the environment variable `STREAMINGFAST_KEY` (you can find it on the Streamingfast dashboard), and run `. ./token.sh`.

Finally, to start the stream, run `make stream START=<slot>`. You might wanna test it out with recent slots first.

Note that currently only swap transactions are parsed.
