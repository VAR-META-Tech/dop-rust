mod dop;
use dop::DopClient;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = DopClient::new();
    engine.start();
    engine.wait_for_api_ready().await;

    engine
        .init_engine(
            Some("database/test-key-addr.db"),
            Some("KeyAddr Engine"),
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
    let id = wallet_info["id"].as_str().expect("Missing wallet ID");

    // let private_key = engine.get_private_viewing_key(id).await?;
    // println!("Private Key: {:?}", private_key);
    // assert!(
    //     !private_key.is_empty(),
    //     "Private viewing key should not be empty"
    // );

    let dop_address = engine.get_dop_address(id).await?;
    println!("DOP Address: {:?}", dop_address);

    let address_data = engine.get_dop_wallet_address_data(&dop_address).await?;
    println!("Address Data: {}", address_data);

    engine.close_engine().await?;
    Ok(())
}
