use anyhow::Result;
use dop::dop::{
    DopClient, DopERC20AmountRecipient, DopNFTAmountRecipient, FeeTokenDetails,
    TransactionGasDetails,
};
use serde_json::json;
use serial_test::serial;

async fn setup_engine() -> Result<DopClient> {
    let mut engine = DopClient::new();
    engine.start();
    engine.wait_for_api_ready().await;
    engine.init_engine(None, None, None, None, None).await?;

    let fallback_providers = json!({
        "chainId": 11155111,
        "providers": [
            {
                "provider": "https://sepolia.drpc.org",
                "priority": 3,
                "weight": 3,
                "maxLogsPerBatch": 2,
                "stallTimeout": 2500
            },
            {
                "provider": "https://ethereum-sepolia-rpc.publicnode.com",
                "priority": 3,
                "weight": 2,
                "maxLogsPerBatch": 5
            }
        ]
    });

    engine
        .load_provider(fallback_providers, "Ethereum_Sepolia", Some(10_000))
        .await?;
    println!("✅ Provider loaded");

    let chain = json!({ "type": 0, "id": 11155111 });
    engine.scan_contract_history(chain, None).await?;
    println!("✅ scan_contract_history success");

    Ok(engine)
}

#[tokio::test]
#[serial]
async fn test_gas_estimate_for_unproven_transfer() -> Result<()> {
    let mut engine = setup_engine().await?;
    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";
    let wallet_info = engine
        .create_wallet(&mnemonic, encryption_key, None)
        .await?;
    let dop_wallet_id = wallet_info["id"].as_str().unwrap().to_string();
    let dop_address = wallet_info["dopAddress"].as_str().unwrap().to_string();

    let wallet = engine.get_wallet(&dop_wallet_id).await?;
    println!("✅ Wallet: {:?}", wallet);

    let recipients = vec![DopERC20AmountRecipient {
        token_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
        amount: "1000".to_string(),
        recipient_address: dop_address.clone(),
    }];

    let gas_details = TransactionGasDetails::Type2 {
        gas_estimate: "500000".to_string(),
        max_fee_per_gas: "10000000000".to_string(),
        max_priority_fee_per_gas: "1000000000".to_string(),
    };

    let result = engine
        .gas_estimate_for_unproven_transfer(
            "V2_PoseidonMerkle".to_string(),
            "Ethereum_Sepolia".to_string(),
            dop_wallet_id,
            encryption_key.to_string(),
            None,
            recipients,
            vec![],
            gas_details,
            None::<FeeTokenDetails>,
            false,
        )
        .await;

    match result {
        Ok(estimate) => {
            println!("✅ Gas Estimate: {:?}", estimate.gas_estimate);
        }
        Err(err) => {
            println!("⚠️ Expected error: {:?}", err);
        }
    }

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_populate_proved_transfer() -> Result<()> {
    let mut engine = setup_engine().await?;
    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";
    let wallet_info = engine
        .create_wallet(&mnemonic, encryption_key, None)
        .await?;
    let dop_wallet_id = wallet_info["id"].as_str().unwrap().to_string();
    let dop_address = wallet_info["dopAddress"].as_str().unwrap().to_string();

    let recipients = vec![DopERC20AmountRecipient {
        token_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
        amount: "1000".to_string(),
        recipient_address: dop_address.clone(),
    }];

    let gas_details = TransactionGasDetails::Type2 {
        gas_estimate: "500000".to_string(),
        max_fee_per_gas: "10000000000".to_string(),
        max_priority_fee_per_gas: "1000000000".to_string(),
    };

    let result = engine
        .populate_proved_transfer(
            "V2_PoseidonMerkle".to_string(),
            "Ethereum_Sepolia".to_string(),
            dop_wallet_id,
            false,
            None,
            recipients,
            vec![],
            None,
            false,
            None,
            gas_details,
        )
        .await;

    match result {
        Ok(tx) => {
            println!(
                "✅ Populated proved transfer: to = {}, data length = {}",
                tx.transaction.to,
                tx.transaction.data.len()
            );
        }
        Err(err) => {
            println!("⚠️ Expected error: {:?}", err);
        }
    }

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_generate_transfer_proof() -> Result<()> {
    let mut engine = setup_engine().await?;
    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";
    let wallet_info = engine
        .create_wallet(&mnemonic, encryption_key, None)
        .await?;
    let dop_wallet_id = wallet_info["id"].as_str().unwrap().to_string();
    let dop_address = wallet_info["dopAddress"].as_str().unwrap().to_string();

    let recipients = vec![DopERC20AmountRecipient {
        token_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
        amount: "1000".to_string(),
        recipient_address: dop_address.clone(),
    }];

    let result = engine
        .generate_transfer_proof(
            "V2_PoseidonMerkle".to_string(),
            "Ethereum_Sepolia".to_string(),
            dop_wallet_id,
            encryption_key.to_string(),
            false,
            None,
            recipients,
            vec![],
            None,
            false,
            None,
        )
        .await;

    match result {
        Ok(_) => println!("✅ Transfer proof generation complete"),
        Err(err) => println!("❌ Failed to generate transfer proof: {}", err),
    }

    engine.close_engine().await?;
    Ok(())
}
