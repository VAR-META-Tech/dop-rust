mod dop;
use dop::DopClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = DopClient::new();
    engine.start();
    engine.wait_for_api_ready().await;
    println!("✅ Node.js API is HERE");
    engine.init_engine(None, None, None, None, None).await?;

    println!("✅ BEFORE GENERATE MNEMONIC");

    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

    let wallet_info = engine
        .create_wallet(&mnemonic, encryption_key, None)
        .await?;
    let _dop_address = wallet_info["dopAddress"]
        .as_str()
        .expect("Missing dopAddress in walletInfo");

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

    engine.scan_contract_history(chain.clone(), None).await?;
    println!("✅ scan_contract_history success");

    engine.close_engine().await?;
    Ok(())
}
