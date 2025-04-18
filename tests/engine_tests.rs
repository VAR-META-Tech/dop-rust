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
    let status = engine.engine_status().await?;
    assert_eq!(status, "READY", "Engine status should be READY after initialization");

    engine.close_engine().await?;
    Ok(())
}
