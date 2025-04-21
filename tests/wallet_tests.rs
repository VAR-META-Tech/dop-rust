use dop::engine::DopEngine;
use std::collections::HashMap;
use serial_test::serial;
use anyhow::Result;


#[tokio::test]
#[serial]
async fn test_generate_create_and_get_wallet() -> Result<(), anyhow::Error> {
    let mut engine = DopEngine::new();
    engine.start();
    engine.wait_for_api_ready().await;

    engine
        .init_engine(
            Some("database/DOP.db"),
            Some("DOP Engine"),
            Some(false),
            Some(false),
            Some(false),
        )
        .await?;

    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    assert!(!mnemonic.is_empty(), "Generated mnemonic should not be empty");

    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

    let creation_blocks = Some(HashMap::from([
        ("Ethereum", 0u64),
        ("Polygon", 2u64),
    ]));

    let wallet_info = engine
        .create_wallet(&mnemonic, encryption_key, creation_blocks)
        .await?;

    let id = wallet_info
        .get("id")
        .and_then(|v| v.as_str())
        .expect("Missing wallet ID");

    let wallet = engine.get_wallet(id).await?;
    assert_eq!(wallet["id"], id, "Wallet ID should match");

    engine.close_engine().await?;
    Ok(())
}
#[tokio::test]
#[serial]
async fn test_shareable_viewing_key_and_view_only_wallet() -> Result<(), anyhow::Error> {
      let mut engine = DopEngine::new();
    engine.start();
    engine.wait_for_api_ready().await;

    engine
        .init_engine(
            Some("database/DOP.db"),
            Some("DOP Engine"),
            Some(false),
            Some(true),
            Some(false),
        )
        .await?;

    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

    let wallet_info = engine
        .create_wallet(&mnemonic, encryption_key, None)
        .await?;

    let id = wallet_info
        .get("id")
        .and_then(|v| v.as_str())
        .expect("Missing wallet ID");

    let shareable_key = engine.get_shareable_viewing_key(id).await?;
    assert!(!shareable_key.is_empty(), "Viewing key should not be empty");

    let creation_blocks = Some(HashMap::from([
        ("Ethereum", 0u64),
        ("Polygon", 2u64),
    ]));

    let view_only_wallet = engine
        .create_view_only_wallet(encryption_key, &shareable_key, creation_blocks)
        .await?;

    assert!(view_only_wallet.get("id").is_some(), "View-only wallet should have an ID");

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_wallet_mnemonic() -> Result<()> {
    let mut engine = DopEngine::new();
    engine.start();
    engine.wait_for_api_ready().await;

    engine.init_engine(
        Some("database/test-mnemonic.db"),
        Some("Mnemonic Engine"),
        Some(false),
        Some(true),
        Some(false),
    ).await?;

    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

    let wallet_info = engine.create_wallet(&mnemonic, encryption_key, None).await?;
    let id = wallet_info["id"].as_str().expect("Missing wallet ID");

    let restored_mnemonic = engine.get_wallet_mnemonic(id,encryption_key).await?;
    println!("Restored Mnemonic: {}", restored_mnemonic);
    assert_eq!(mnemonic, restored_mnemonic, "Restored mnemonic does not match");

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_sign_with_wallet_viewing_key() -> Result<()> {
    let mut engine = DopEngine::new();
    engine.start();
    engine.wait_for_api_ready().await;

    engine.init_engine(
        Some("database/test-sign.db"),
        Some("Sign Engine"),
        Some(false),
        Some(true),
        Some(false),
    ).await?;

    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

    let wallet_info = engine.create_wallet(&mnemonic, encryption_key, None).await?;
    let id = wallet_info["id"].as_str().expect("Missing wallet ID");
    println!("Wallet ID: {}", id);
    let message = "Hello DOP!";
    let signature = engine.sign_message_with_wallet(id, message).await?;

    println!("Signature: {}", signature);
    assert!(!signature.is_empty(), "Signature should not be empty");

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_load_wallet_by_id() -> Result<(), anyhow::Error> {
    let mut engine = DopEngine::new();
    engine.start();
    engine.wait_for_api_ready().await;

    engine.init_engine(
        Some("database/test-load.db"),
        Some("Load Engine"),
        Some(false),
        Some(true),
        Some(false),
    ).await?;

    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

    let wallet_info = engine.create_wallet(&mnemonic, encryption_key, None).await?;
    let id = wallet_info["id"].as_str().expect("Missing wallet ID");

    let loaded_wallet = engine.load_wallet_by_id(encryption_key, id, false).await?;
    assert_eq!(loaded_wallet["id"], id, "Loaded wallet ID mismatch");

    engine.close_engine().await?;
    Ok(())
}