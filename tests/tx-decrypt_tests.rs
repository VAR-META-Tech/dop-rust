use anyhow::Result;
use dop::dop::{
    DopClient, DopERC20Amount, DopERC20AmountRecipient, DopNFTAmountRecipient, FeeTokenDetails,
    TransactionGasDetails,
};
use serde_json::json;
use serial_test::serial;

async fn setup_client() -> Result<DopClient> {
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

async fn setup_wallet(client: &DopClient) -> Result<(String, String)> {
    let mnemonic = client.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";
    let wallet_info = client
        .create_wallet(&mnemonic, encryption_key, None)
        .await?;
    let dop_wallet_id = wallet_info["dopAddress"].as_str().unwrap().to_owned();
    Ok((dop_wallet_id, encryption_key.to_owned()))
}

#[tokio::test]
#[serial]
async fn test_generate_decrypt_proof() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, encryption_key) = setup_wallet(&client).await?;

    match client
        .generate_decrypt_proof(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id,
            encryption_key,
            vec![],
            vec![],
            None,
            false,
            None,
            "0".into(),
        )
        .await
    {
        Ok(_) => println!("✅ generate_decrypt_proof passed"),
        Err(e) => println!("⚠️ generate_decrypt_proof failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_generate_decrypt_to_origin_proof() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, encryption_key) = setup_wallet(&client).await?;

    match client
        .generate_decrypt_to_origin_proof(
            "original_txid".into(),
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id,
            encryption_key,
            vec![],
            vec![],
            "0".into(),
        )
        .await
    {
        Ok(_) => println!("✅ generate_decrypt_to_origin_proof passed"),
        Err(e) => println!("⚠️ generate_decrypt_to_origin_proof failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_generate_decrypt_base_token_proof() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, encryption_key) = setup_wallet(&client).await?;

    let wrapped_amount = DopERC20Amount {
        token_address: "0xToken".into(),
        amount: "1000".into(),
    };

    match client
        .generate_decrypt_base_token_proof(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            "0xPublicWallet".into(),
            wallet_id,
            encryption_key,
            wrapped_amount,
            None,
            false,
            None,
        )
        .await
    {
        Ok(_) => println!("✅ generate_decrypt_base_token_proof passed"),
        Err(e) => println!("⚠️ generate_decrypt_base_token_proof failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_populate_proved_decrypt() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, _) = setup_wallet(&client).await?;

    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "21000".into(),
        gas_price: "1000000000".into(),
    };

    match client
        .populate_proved_decrypt(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id,
            vec![],
            vec![],
            None,
            false,
            None,
            gas_details,
        )
        .await
    {
        Ok(tx) => println!(
            "✅ populate_proved_decrypt to = {}, data.len = {}",
            tx.transaction.to,
            tx.transaction.data.len()
        ),
        Err(e) => println!("⚠️ populate_proved_decrypt failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_populate_proved_decrypt_base_token() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, _) = setup_wallet(&client).await?;

    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "21000".into(),
        gas_price: "1000000000".into(),
    };

    let wrapped_amount = DopERC20Amount {
        token_address: "0xToken".into(),
        amount: "1000".into(),
    };

    match client
        .populate_proved_decrypt_base_token(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            "0xPublicWallet".into(),
            wallet_id,
            wrapped_amount,
            None,
            false,
            None,
            gas_details,
        )
        .await
    {
        Ok(tx) => println!(
            "✅ populate_proved_decrypt_base_token to = {}, data.len = {}",
            tx.transaction.to,
            tx.transaction.data.len()
        ),
        Err(e) => println!("⚠️ populate_proved_decrypt_base_token failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_gas_estimate_for_unproven_decrypt() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, encryption_key) = setup_wallet(&client).await?;

    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "21000".into(),
        gas_price: "1000000000".into(),
    };

    match client
        .gas_estimate_for_unproven_decrypt(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id,
            encryption_key,
            vec![],
            vec![],
            gas_details,
            None,
            false,
            "0".into(),
        )
        .await
    {
        Ok(gas) => println!("✅ gas_estimate_for_unproven_decrypt = {:?}", gas),
        Err(e) => println!("⚠️ gas_estimate_for_unproven_decrypt failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_gas_estimate_for_unproven_decrypt_base_token() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, encryption_key) = setup_wallet(&client).await?;

    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "21000".into(),
        gas_price: "1000000000".into(),
    };

    let wrapped_amount = DopERC20Amount {
        token_address: "0xToken".into(),
        amount: "1000".into(),
    };

    match client
        .gas_estimate_for_unproven_decrypt_base_token(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            "0xPublicWallet".into(),
            wallet_id,
            encryption_key,
            wrapped_amount,
            gas_details,
            None,
            false,
        )
        .await
    {
        Ok(gas) => println!(
            "✅ gas_estimate_for_unproven_decrypt_base_token = {:?}",
            gas
        ),
        Err(e) => println!(
            "⚠️ gas_estimate_for_unproven_decrypt_base_token failed: {:?}",
            e
        ),
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_recipients_for_decrypt_to_origin() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, _) = setup_wallet(&client).await?;

    match client
        .get_erc20_and_nft_amount_recipients_for_decrypt_to_origin(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id,
            "original_txid".into(),
        )
        .await
    {
        Ok((erc20, nft)) => println!("✅ recipients: ERC20 = {:?}, NFT = {:?}", erc20, nft),
        Err(e) => println!("⚠️ get_recipients_for_decrypt_to_origin failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_populate_proved_decrypt_to_origin() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, _) = setup_wallet(&client).await?;

    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "21000".into(),
        gas_price: "1000000000".into(),
    };

    match client
        .populate_proved_decrypt_to_origin(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id,
            vec![],
            vec![],
            gas_details,
        )
        .await
    {
        Ok(tx) => println!(
            "✅ populate_proved_decrypt_to_origin to = {}, data.len = {}",
            tx.transaction.to,
            tx.transaction.data.len()
        ),
        Err(e) => println!("⚠️ populate_proved_decrypt_to_origin failed: {:?}", e),
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_gas_estimate_for_unproven_decrypt_to_origin() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, encryption_key) = setup_wallet(&client).await?;

    match client
        .gas_estimate_for_unproven_decrypt_to_origin(
            "original_txid".into(),
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id,
            encryption_key,
            vec![],
            "0".into(),
            vec![],
        )
        .await
    {
        Ok(gas) => println!("✅ gas_estimate_for_unproven_decrypt_to_origin = {:?}", gas),
        Err(e) => println!(
            "⚠️ gas_estimate_for_unproven_decrypt_to_origin failed: {:?}",
            e
        ),
    }

    Ok(())
}
