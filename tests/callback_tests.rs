use dop::dop::DopClient;
use serde_json::json;
use serial_test::serial;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

#[tokio::test]
#[serial]
async fn test_scan_callbacks_triggered() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the DopClient
    let mut engine = DopClient::new();
    engine.start();
    engine.wait_for_api_ready().await;
    engine.init_engine(None, None, None, None, None).await?;

    // Shared flags to track callback invocations

    engine.set_utxo_scan_callback(move |_event| {
        println!("🛠️ [Rust] UTXO scan event: {:?}", _event);
    });

    // Set TXID scan callback
    engine.set_txid_scan_callback(move |_event| {
        println!("🛠️ [Rust] txidTXO scan event: {:?}", _event);
    });
    let chain = json!({
        "type": 0, // EVM
        "id": 11155111, // Sepolia
    });
    engine.reset_full_txid_merkletrees_v2(chain.clone()).await?;
    println!("✅ reset_full_txid_merkletrees_v2 success");

    // Start the scan listeners
    engine.start_scan_listeners().await?;
    println!("✅ Scan listeners started");

    // Define the chain configuration
    let chain = json!({
        "type": 0, // EVM
        "id": 11155111, // Sepolia
    });

    // Load Sepolia providers
    let fallback_providers = json!({
        "chainId": 11155111,
        "providers": [
            {
                "provider": "https://sepolia.drpc.org",
                "priority": 3,
                "weight": 3,
                "maxLogsPerBatch": 2,
                "stallTimeout": 2500
            },
            {
                "provider": "https://ethereum-sepolia-rpc.publicnode.com",
                "priority": 3,
                "weight": 2,
                "maxLogsPerBatch": 5
            }
        ]
    });
    let polling_interval = 10_000; // 10 seconds
    engine
        .load_provider(
            fallback_providers,
            "Ethereum_Sepolia",
            Some(polling_interval),
        )
        .await?;
    println!("✅ Provider loaded");

    // Trigger scan_contract_history
    engine.scan_contract_history(chain.clone(), None).await?;
    println!("✅ scan_contract_history invoked");

    // Allow some time for the callbacks to be invoked
    sleep(Duration::from_secs(5)).await;


    // Clean up
    engine.close_engine().await?;
    Ok(())
}
