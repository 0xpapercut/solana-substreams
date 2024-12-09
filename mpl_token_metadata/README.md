# metaplex-substream
Stream Metaplex Token Metadata events with [substreams](https://substreams.streamingfast.io).

## Usage
```bash
substreams gui mpl-token-metadaata-events
```
If you see no output, please check that you have set a starting block, e.g. `substreams gui mpl-token-metadaata-events -s 300000000`.

## Disclaimer
Because of the complexity of the metadata program, I will not expand this particular substream. It's generally better to just account data directly from an RPC (if you need just state data).
