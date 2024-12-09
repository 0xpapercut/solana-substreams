# solana-substreams
Solana substreams monorepo.

## Getting started

Try out a module directly from the command line with `substreams`:

```bash
# System Program
substreams gui system-program-events
# SPL Token
substream gui spl-token-events
# Raydium AMM
substreams gui raydium-amm-events
# Pumpfun
substreams gui pumpfun-events
# MPL Token Metadata
substreams gui mpl-token-metadata-events
```

You can access the substreams in this repo either by specifying them as a dependency through `substreams.yaml`, or by using them as libraries (see setup).

## Setup
### Library usage
```toml
[dependencies]
substreams-solana-utils = { git = "https://github.com/0xpapercut/substreams-solana-utils", tag = "v0.1.5" } # Mandatory
system-program-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.5" }
spl-token-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.5" }
raydium-amm-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.5" }
pumpfun-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.5" }
mpl-token-metadata-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.5" }
```

For a realistic example, checkout [solana-indexer](https://github.com/0xpapercut/solana-indexer).
