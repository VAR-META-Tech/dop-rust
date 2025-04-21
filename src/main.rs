mod engine;
use engine::DopEngine;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = DopEngine::new();

    engine.start();
    engine.wait_for_api_ready().await;

    // ✅ Initialize engine with dynamic config
    engine
        .init_engine(
            Some("database/DOP.db"),          // db path
            Some("Rust Init Engine"), // engine name
            Some(false),              // shouldDebug
            Some(false),              // useNativeArtifacts
            Some(false),              // skipMerkletreeScans
        )
        .await?;

    // println!("Engine Status: {}", engine.engine_status().await?);
    let info = engine.get_engine_info().await?;
    assert!(info.get("wallets").is_some(), "Engine info should include wallets field");

    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

    let wallet_info = engine.create_wallet(&mnemonic, encryption_key, None).await?;
    let id = wallet_info["id"].as_str().expect("Missing wallet ID");
    println!("Wallet ID: {}", id);
    let message = "Hello DOP!";
    let signature = engine.sign_message_with_wallet(id, message).await?;

    println!("Signature: {}", signature);
    // ✅ Call close explicitly
    engine.close_engine().await?;
    Ok(())
}
