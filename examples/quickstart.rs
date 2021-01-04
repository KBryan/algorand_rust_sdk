use algo_rust_sdk::{AlgodClient, KmdClient, IndexerClient};

fn main() {
    let algod_address = "http://localhost:4001";
    let algod_token="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    let kmd_address = "http://localhost:4002";
    let kmd_token = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    let indexer_address = "http://localhost:8980";
    let indexer_token = "";


    let algod_client = AlgodClient::new(algod_address, algod_token);
    let kmd_client = KmdClient::new(kmd_address, kmd_token);
    let indexer_client = IndexerClient::new(indexer_address,indexer_token);
    println!(
        "Algod versions: {:?}",
        algod_client.versions().unwrap().versions
    );

    println!(
        "Kmd versions: {:?}",
        kmd_client.versions().unwrap().versions
    );
}
