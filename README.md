# solana-substreams
Solana substreams monorepo.

## Getting started

Try out a module directly from the command line with `substreams`:

```bash
# System Program
substreams run -e mainnet.sol.streamingfast.io:443 https://github.com/0xpapercut/solana-substreams/releases/download/v0.1.4/system-program-events-v0.1.4.spkg system_program_events -s 300000000
# SPL Token
substreams run -e mainnet.sol.streamingfast.io:443 https://github.com/0xpapercut/solana-substreams/releases/download/v0.1.4/spl-token-events-v0.1.4.spkg spl_token_events -s 300000000
# Raydium AMM
substreams run -e mainnet.sol.streamingfast.io:443 https://github.com/0xpapercut/solana-substreams/releases/download/v0.1.4/raydium-amm-events-v0.1.4.spkg raydium_amm_events -s 300000000
# Pumpfun
substreams run -e mainnet.sol.streamingfast.io:443 https://github.com/0xpapercut/solana-substreams/releases/download/v0.1.4/pumpfun-events-v0.1.4.spkg pumpfun_events -s 300000000
# MPL Token Metadata
substreams run -e mainnet.sol.streamingfast.io:443 https://github.com/0xpapercut/solana-substreams/releases/download/v0.1.4/mpl-token-metadata-events-v0.1.4.spkg mpl_token_metadata_events -s 300000000
```

You can access the substreams in this repo either by specifying them as a dependency through `substreams.yaml`, or by using them as libraries (see setup).

## Setup
### Library usage
```toml
[dependencies]
substreams-solana-utils = { git = "https://github.com/0xpapercut/substreams-solana-utils", tag = "v0.1.4" } # Mandatory
system-program-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.4" }
spl-token-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.4" }
raydium-amm-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.4" }
pumpfun-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.4" }
mpl-token-metadata-substream = { git = "https://github.com/0xpapercut/solana-substreams", tag = "v0.1.4" }
```

For a realistic example, checkout [solana-indexer](https://github.com/0xpapercut/solana-indexer).
