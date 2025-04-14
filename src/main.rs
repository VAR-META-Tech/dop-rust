mod engine;
use engine::DopEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = DopEngine::new();

    engine.start();
    engine.wait_for_api_ready().await;

    engine.init_engine().await?;
    println!("Engine Status: {}", engine.engine_status().await?);

    let wallet_info = engine
        .create_wallet(
            "pause crystal tornado alcohol genre cement fade large song like bag where",
            "0101010101010101010101010101010101010101010101010101010101010101",
        )
        .await?;
    println!("Created Wallet: {:#?}", wallet_info);
    println!("Finishing wallet creation...");
    if let Some(id) = wallet_info.get("id").and_then(|v| v.as_str()) {
        let wallet = engine.get_wallet(id).await?;
        println!("Wallet Detail: {:#?}", wallet);
    }

    engine.close_engine().await?;
    engine.stop();

    Ok(())
}
