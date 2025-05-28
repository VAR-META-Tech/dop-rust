use anyhow::Result;
use dop::dop::{DopClient, DopERC20Amount, DopERC20AmountRecipient, DopNFTAmountRecipient};
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

    let chain = json!({ "type": 0, "id": 11155111 });
    engine.scan_contract_history(chain, None).await?;

    Ok(engine)
}

async fn create_wallet(engine: &DopClient) -> Result<(String, String)> {
    let mnemonic = engine.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";
    let wallet_info = engine
        .create_wallet(&mnemonic, encryption_key, None)
        .await?;
    let dop_address = wallet_info["dopAddress"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing dopAddress"))?
        .to_string();
    Ok((dop_address, encryption_key.to_string()))
}

#[tokio::test]
#[serial]
async fn test_gas_estimate_for_encrypt_base_token() -> Result<()> {
    let mut engine = setup_engine().await?;
    let (dop_address, _) = create_wallet(&engine).await?;

    let result = engine
        .gas_estimate_for_encrypt_base_token(
            "V2_PoseidonMerkle".to_string(),
            "Ethereum_Sepolia".to_string(),
            dop_address.clone(),
            "0x0101010101010101010101010101010101010101010101010101010101010101".to_string(),
            DopERC20Amount {
                token_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
                amount: "1000".to_string(),
            },
            "0x9E9F988356f46744Ee0374A17a5Fa1a3A3cC3777".to_string(), // still EVM address for fromWallet
        )
        .await;

    match result {
        Ok(estimate) => {
            println!("✅ Gas Estimate: {:?}", estimate.gas_estimate);
        }
        Err(err) => {
            println!("⚠️ Expected error: {:?}", err);
            assert!(
                format!("{:?}", err).contains("insufficient funds")
                    || format!("{:?}", err).contains("Invalid DOP address"),
                "Expected insufficient funds or invalid dop address error"
            );
        }
    }

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_populate_encrypt_base_token() -> Result<()> {
    let mut engine = setup_engine().await?;
    let (dop_address, _) = create_wallet(&engine).await?;

    let result = engine
        .populate_encrypt_base_token(
            "V2_PoseidonMerkle".to_string(),
            "Ethereum_Sepolia".to_string(),
            dop_address.clone(),
            "0x0101010101010101010101010101010101010101010101010101010101010101".to_string(),
            DopERC20Amount {
                token_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
                amount: "1000".to_string(),
            },
            "0x9E9F988356f46744Ee0374A17a5Fa1a3A3cC3777".to_string(),
            None,
        )
        .await;

    match result {
        Ok(tx) => {
            println!(
                "✅ Populated transaction: to = {}, data length = {}",
                tx.transaction.to,
                tx.transaction.data.len()
            );
        }
        Err(err) => {
            println!("⚠️ Expected error: {:?}", err);
            assert!(
                format!("{:?}", err).contains("insufficient funds")
                    || format!("{:?}", err).contains("Invalid DOP address"),
                "Expected insufficient funds or invalid dop address error"
            );
        }
    }

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_gas_estimate_for_encrypt() -> Result<()> {
    let mut engine = setup_engine().await?;
    let (dop_address, encryption_key) = create_wallet(&engine).await?;

    let recipients = vec![DopERC20AmountRecipient {
        token_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
        amount: "1000".to_string(),
        recipient_address: dop_address.clone(), // ✅ DOP Address
    }];

    let result = engine
        .gas_estimate_for_encrypt(
            "V2_PoseidonMerkle".to_string(),
            "Ethereum_Sepolia".to_string(),
            encryption_key.clone(),
            recipients,
            vec![], // no NFTs
            "0x9E9F988356f46744Ee0374A17a5Fa1a3A3cC3777".to_string(),
        )
        .await;

    match result {
        Ok(estimate) => {
            println!("✅ Gas Estimate: {:?}", estimate.gas_estimate);
        }
        Err(err) => {
            println!("⚠️ Expected error: {:?}", err);
            assert!(
                format!("{:?}", err).contains("insufficient funds")
                    || format!("{:?}", err).contains("Invalid DOP address")
                    || format!("{:?}", err).contains("RPC connection error"),
                "Expected insufficient funds, invalid dop address, or rpc connection error"
            );
        }
    }

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_populate_encrypt() -> Result<()> {
    let mut engine = setup_engine().await?;
    let (dop_address, encryption_key) = create_wallet(&engine).await?;

    let recipients = vec![DopERC20AmountRecipient {
        token_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
        amount: "1000".to_string(),
        recipient_address: dop_address.clone(), // ✅ DOP Address
    }];

    let result = engine
        .populate_encrypt(
            "V2_PoseidonMerkle".to_string(),
            "Ethereum_Sepolia".to_string(),
            encryption_key.clone(),
            recipients,
            vec![],
            None,
        )
        .await;

    match result {
        Ok(tx) => {
            println!(
                "✅ Populated transaction: to = {}, data length = {}",
                tx.transaction.to,
                tx.transaction.data.len()
            );
        }
        Err(err) => {
            println!("⚠️ Expected error: {:?}", err);
            assert!(
                format!("{:?}", err).contains("insufficient funds")
                    || format!("{:?}", err).contains("Invalid DOP address"),
                "Expected insufficient funds or invalid dop address error"
            );
        }
    }

    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_encrypt_private_key_signature_message() -> Result<()> {
    let mut engine = setup_engine().await?;

    let message = engine.get_encrypt_private_key_signature_message().await?;
    println!("✅ Signature Message: {}", message);

    assert!(!message.is_empty());
    engine.close_engine().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_generate_encrypt_transaction() -> Result<()> {
    let mut engine = setup_engine().await?;
    let (dop_address, encryption_key) = create_wallet(&engine).await?;

    let recipients = vec![DopERC20AmountRecipient {
        token_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
        amount: "1000".to_string(),
        recipient_address: dop_address.clone(), // ✅ DOP Address
    }];

    let tx = engine
        .generate_encrypt_transaction(
            "V2_PoseidonMerkle".to_string(),
            "Ethereum_Sepolia".to_string(),
            encryption_key,
            recipients,
            vec![],
        )
        .await?;

    println!(
        "✅ Generated Encrypt Transaction: to = {}, data length = {}",
        tx.to,
        tx.data.len()
    );

    assert!(!tx.to.is_empty());
    assert!(!tx.data.is_empty());
    engine.close_engine().await?;
    Ok(())
}
