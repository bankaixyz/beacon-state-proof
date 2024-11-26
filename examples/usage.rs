use beacon_state_proof::state_proof_fetcher::StateProofFetcher;
use beacon_state_proof::state_proof_fetcher::TreeHash;

#[tokio::main]
async fn main() {
    let fetcher = StateProofFetcher::new("http://127.0.0.1:5052".to_string());
    match fetcher.fetch_next_sync_committee_proof(6408035).await {
        Ok(proof) => println!("{:?}", proof.next_sync_committee.tree_hash_root()),
        Err(e) => eprintln!("Error fetching state proof: {:?}", e),
    }
} 