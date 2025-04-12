mod engine;
use engine::DopEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = DopEngine::new();

    engine.start();

    engine.wait_for_api_ready().await;

    engine.init_engine().await?;
    println!("Engine Status: {}", engine.engine_status().await?);

    let engine_info = engine.get_engine_info().await?;
    println!("Engine Info: {:#?}", engine_info);

    engine.close_engine().await?;
    engine.stop();

    Ok(())
}
