# State Proof Fetcher

This library enables the arbitrary fetching of merkle inclusion proofs for any field of the Ethereum beacon chain state. Please be aware that this is a very heavy operation, as it requires downloading the entire beacon state at the given slot, which can be several 100s of MBs in size.

## Features

- Fetch beacon state at a given slot.
- Compute Merkle proofs for specific indices within the state.
- Asynchronous API using `tokio`.
- Custom error handling for easier debugging.

## Installation

Add this to your `Cargo.toml`: 

```toml
[dependencies]

beacon_state_proof = { git = "https://github.com/petscheit/beacon-state-proof", tag = "v0.1.0" }
```


## Usage
```rust
use beacon_state_proof::state_proof_fetcher::StateProofFetcher;

#[tokio::main]
async fn main() {
    let fetcher = StateProofFetcher::new("BEACON_NODE_RPC_URL".to_string());
    match fetcher.fetch_state_proof(6408035, 55).await {
        Ok(proof) => println!("Proof: {:?}", proof),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```