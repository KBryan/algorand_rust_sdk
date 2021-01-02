use std::error::Error;

use algo_rust_sdk::AlgodClientV2;

fn main() -> Result<(), Box<dyn Error>> {
    let algod_address = "http://localhost:4001";
    let algod_token="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    let algod_client = AlgodClientV2::new(algod_address, algod_token);

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
/*
{"catchpoint":"","catchpoint-acquired-blocks":0,"catchpoint-processed-accounts":0,"catchpoint-total-accounts":0,"catchpoint-total-blocks":0,"catchpoint-verified-accounts":0,"catchup-time":0,"last-catchpoint":"","last-round":11475981,"last-version":"https://github.com/algorandfoundation/specs/tree/3a83c4c743f8b17adfd73944b4319c25722a6782","next-version":"https://github.com/algorandfoundation/specs/tree/3a83c4c743f8b17adfd73944b4319c25722a6782","next-version-round":11475982,"next-version-supported":true,"stopped-at-unsupported-round":false,"time-since-last-round":1523593400}
 */