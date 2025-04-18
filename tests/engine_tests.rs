use dop::engine::DopEngine;
use anyhow::Result;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_engine_lifecycle() -> Result<()> {
    let mut engine = DopEngine::new();

    engine.start();
    engine.wait_for_api_ready().await;

    engine.init_engine(None, None, None, None, None).await?;

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_engine_info() -> Result<(), anyhow::Error> {
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

    let info = engine.get_engine_info().await?;
    assert!(info.get("wallets").is_some(), "Engine info should include wallets field");

    engine.close_engine().await?;
    Ok(())
}
