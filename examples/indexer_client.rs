use std::error::Error;

use algo_rust_sdk::IndexerClient;

fn main() -> Result<(), Box<dyn Error>> {
    let indexer_address = "http://localhost:8980";
    let indexer_token="";

    let indexer_client = IndexerClient::new(indexer_address, indexer_token);

     //Print algod status

    let _node_status = indexer_client.health()?;

    Ok(())
}
