use dop::engine::DopEngine;
use std::collections::HashMap;
use serial_test::serial;



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
