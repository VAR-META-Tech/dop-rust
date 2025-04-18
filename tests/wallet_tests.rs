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
