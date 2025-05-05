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
    println!("✅ Provider loaded");

    let chain = json!({ "type": 0, "id": 11155111 });
    engine.scan_contract_history(chain, None).await?;
    println!("✅ scan_contract_history success");

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

    client
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
        .await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_generate_decrypt_to_origin_proof() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, encryption_key) = setup_wallet(&client).await?;

    client
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
        .await?;

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

    client
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
        .await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_populate_proved_decrypt() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, _) = setup_wallet(&client).await?;

    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "21000".to_owned(),
        gas_price: "1000000000".to_owned(),
    };

    let result = client
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
        .await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_populate_proved_decrypt_base_token() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, _) = setup_wallet(&client).await?;

    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "21000".to_owned(),
        gas_price: "1000000000".to_owned(),
    };

    let wrapped_amount = DopERC20Amount {
        token_address: "0xToken".into(),
        amount: "1000".into(),
    };

    let result = client
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
        .await?;

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

    let result = client
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
        .await?;

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

    let result = client
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
        .await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_get_recipients_for_decrypt_to_origin() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, _) = setup_wallet(&client).await?;

    let (erc20, nft) = client
        .get_erc20_and_nft_amount_recipients_for_decrypt_to_origin(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id,
            "original_txid".into(),
        )
        .await?;

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

    let result = client
        .populate_proved_decrypt_to_origin(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id,
            vec![],
            vec![],
            gas_details,
        )
        .await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_gas_estimate_for_unproven_decrypt_to_origin() -> Result<()> {
    let client = setup_client().await?;
    let (wallet_id, encryption_key) = setup_wallet(&client).await?;

    let result = client
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
        .await?;

    Ok(())
}
