# rust algorand sdk

# Documentation

# This project is frozen until the next release of TEAL.

Please visit for active project

https://github.com/manuelmauro/algonaut 

This is the current Rust version implementation of the Algorand SDK <br>
General Algorand documentation is available at https://developer.algorand.org/ <br>

Please look at the examples for Rust Algorand usage.<br>
You can also find Algorand Rust documentation at https://docs.rs/algo_rust_sdk/1.0.3/algo_rust_sdk/</br>
This repo is in RAPID development and subject to breaking changes</br>
and doesn't mirror this repo please visit https://crates.io/crates/algo_rust_sdk </br>
For stable release candidate and documentation.



# Quickstart
This quick start guide assumes the user has the Algorand Sandbox 2.0 installed.<br>
and using this repo as source.

```rust
use std::error::Error;

use algo_rust_sdk::AlgodClient;

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
