mod dop;
use dop::{DopClient, DopERC20Amount};
use serde_json::json;
use std::{collections::HashMap, ffi::c_long};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = DopClient::new();
    engine.start();
    engine.wait_for_api_ready().await;
    engine.init_engine(None, None, None, None, None).await?;

    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

    let wallet_info = engine
        .create_wallet(&mnemonic, encryption_key, None)
        .await?;
    let dop_address = wallet_info["dopAddress"]
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

    let estimate = engine
        .gas_estimate_for_encrypt_base_token(
            "V2_PoseidonMerkle".to_string(),
            "Ethereum_Sepolia".to_string(),
            dop_address.to_string(),
            "e4f9d8a6eaa57db1f82b9bbebd22e67468221b76df2ef7d8377ef2b9e8d6e74d".to_string(),
            DopERC20Amount {
                token_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
                amount: "1000".to_string(),
            },
            "0x9E9F988356f46744Ee0374A17a5Fa1a3A3cC3777".to_string(),
        )
        .await
        .expect("Gas estimate failed");

    println!("Gas Estimate: {:?}", estimate.gas_estimate);

    engine.close_engine().await?;
    Ok(())
}
