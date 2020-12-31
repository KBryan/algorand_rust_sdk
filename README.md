# rust algorand sdk

# Documentation

This is the current attempt to stay up Rust version of the Algorand SDK <br>
General Algorand documentation is available at https://developer.algorand.org/


# Quickstart
This quick start guide assumes the user has the Algorand Sandbox 2.0 installed.

```rust
use std::error::Error;

use algosdk::AlgodClient;

fn main() -> Result<(), Box<dyn Error>> {
    let algod_address = "http://localhost:4001";
    let algod_token="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    let algod_client = AlgodClient::new(algod_address, algod_token);

    // Print algod status
    let node_status = algod_client.status()?;
    println!("algod last round: {}", node_status.last_round);
    println!(
        "algod time since last round: {}",
        node_status.time_since_last_round
    );
    println!("algod catchup: {}", node_status.catchup_time);
    println!("algod latest version: {}", node_status.last_version);

    // Fetch block information
    let last_block = algod_client.block(node_status.last_round)?;
    println!("{:#?}", last_block);

    Ok(())
}

```

