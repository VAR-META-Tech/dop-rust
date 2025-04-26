use anyhow::Result;
use dop::dop::DopClient;
use serde_json::json;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_balance_operations() -> Result<()> {
    let mut engine = DopClient::new();
    engine.start();
    engine.wait_for_api_ready().await;
    engine.init_engine(None, None, None, None, None).await?;

    // Sepolia config
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
    let polling_interval = 10_000; // 1 minute
    engine
        .load_provider(
            fallback_providers,
            "Ethereum_Sepolia",
            Some(polling_interval),
        )
        .await?;
    println!("✅ Provider loaded");

    // 1. Test refresh_balances
    engine.refresh_balances(chain.clone(), None).await?;
    println!("✅ refresh_balances success");

    // 2. Test reset_full_txid_merkletrees_v2
    engine.reset_full_txid_merkletrees_v2(chain.clone()).await?;
    println!("✅ reset_full_txid_merkletrees_v2 success");

    // 3. Test scan_contract_history
    engine.scan_contract_history(chain.clone(), None).await?;
    println!("✅ scan_contract_history success");

    // 4. Test rescan_full_utxo_merkletrees_and_wallets
    engine
        .rescan_full_utxo_merkletrees_and_wallets(chain.clone(), None)
        .await?;
    println!("✅ rescan_full_utxo_merkletrees_and_wallets success");

    engine.close_engine().await?;
    Ok(())
}
