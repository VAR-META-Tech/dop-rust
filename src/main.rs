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
    // ✅ Generate a new mnemonic with 12 words
    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    println!("Generated Mnemonic: {}", mnemonic);

    let encryption_key =
        "0101010101010101010101010101010101010101010101010101010101010101";

    // ✅ Optional creation block numbers
    let creation_block_numbers = Some(HashMap::from([
        ("Ethereum", 0u64),
        ("Polygon", 2u64),
    ]));

    // ✅ Create wallet
    let wallet_info = engine
        .create_wallet(&mnemonic, encryption_key, creation_block_numbers)
        .await?;
    println!("Created Wallet: {:#?}", wallet_info);

    // ✅ Retrieve full wallet by ID
    if let Some(id) = wallet_info.get("id").and_then(|v| v.as_str()) {
        let wallet = engine.get_wallet(id).await?;
        println!("Wallet Detail: {:#?}", wallet);
    }

    // ✅ Call close explicitly
    engine.close_engine().await?;
    Ok(())
}
