use dop::engine::DopEngine;
use anyhow::Result;
use serde_json::json;
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
#[tokio::test]
#[serial]
async fn test_set_loggers() -> Result<()> {
    let mut engine = DopEngine::new();

    engine.start();
    engine.wait_for_api_ready().await;

    engine
        .init_engine(
            Some("database/DOP.db"),
            Some("DOP Engine"),
            Some(true),  // shouldDebug enabled
            Some(true),  // useNativeArtifacts
            Some(false),
        )
        .await?;

    // ✅ Set logger
    let result = engine.set_loggers().await;
    println!("Set loggers result: {:#?}", result);
    assert!(result.is_ok(), "Failed to set loggers");

    engine.close_engine().await?;
    Ok(())
}


// #[tokio::test]
// #[serial]
// async fn test_load_provider() -> Result<(), anyhow::Error> {
//     let mut engine = DopEngine::new();
//     engine.start();
//     engine.wait_for_api_ready().await;

//     engine
//         .init_engine(
//             Some("database/DOP.db"),
//             Some("DOP Engine"),
//             Some(false),
//             Some(true),
//             Some(false),
//         )
//         .await?;

//     let config = serde_json::json!({
//         "chainId": 137,
//         "providers": [
//             {
//                 "provider": "https://light-serene-feather.matic.quiknode.pro/f0cdd8c4c146e68ce2f935bba399ca66cbe45868/",
//                 "priority": 1,
//                 "weight": 2,
//                 "maxLogsPerBatch": 10,
//                 "stallTimeout": 2500
//             },
//             {
//                 "provider": "https://polygon-bor.publicnode.com",
//                 "priority": 1,
//                 "weight": 2,
//                 "maxLogsPerBatch": 10,
//                 "stallTimeout": 2500
//             },
//             {
//                 "provider": "https://light-serene-feather.matic.quiknode.pro/f0cdd8c4c146e68ce2f935bba399ca66cbe45868/",
//                 "priority": 2,
//                 "weight": 2,
//                 "maxLogsPerBatch": 10
//             }
//         ]
//     });

//     let result = engine
//         .load_provider(config, "Polygon", 10_000)
//         .await?;

//     println!("Provider loaded: {:#?}", result);
//     assert!(result.is_object(), "Expected JSON object");

//     engine.close_engine().await?;
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_gas_estimate_for_unproven_transfer() -> Result<()> {
//     let mut engine = DopEngine::new();
//     engine.start();
//     engine.wait_for_api_ready().await;

//     engine
//         .init_engine(
//             Some("database/test-gas.db"),
//             Some("Gas Test Engine"),
//             Some(false),
//             Some(true),
//             Some(false),
//         )
//         .await?;

//     let mnemonic = "test test test test test test test test test test test junk";
//     let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

//     let wallet_info = engine
//         .create_wallet(mnemonic, encryption_key, None)
//         .await?;
//     let wallet_id = wallet_info
//         .get("id")
//         .and_then(|v| v.as_str())
//         .expect("Wallet ID missing");
//         println!("Created Wallet: {:#?}", wallet_info);
//     let result = engine
//         .gas_estimate_for_unproven_transfer(
//             "txid_version",
//             "Polygon",
//             wallet_id,
//             encryption_key,
//             "test memo",
//             vec![
//                 json!({
//                     "tokenAddress": "0x5FbDB2315678afecb367f032d93F642f64180aa3",
//                     "amount": "0x100",
//                     "recipientAddress": "0zk1q8hxknrs97q8pjxaagwthzc0df99rzmhl2xnlxmgv9akv32sua0kfrv7j6fe3z53llhxknrs97q8pjxaagwthzc0df99rzmhl2xnlxmgv9akv32sua0kg0zpzts"
//                 })
//             ],
//             vec![],
//             json!({
//                 "evmGasType": 2,
//                 "gasEstimate": 0,
//                 "maxFeePerGas": "0x1234567890",
//                 "maxPriorityFeePerGas": "0x123456"
//             }),
//             json!({
//                 "tokenAddress": "0x5FbDB2315678afecb367f032d93F642f64180aa3",
//                 "feePerUnitGas": "0x2000000000000000000"
//             }),
//             false,
//         )
//         .await?;

//     println!("Gas Estimate Result: {:#?}", result);
//     assert!(result.is_object(), "Expected result to be a JSON object");

//     engine.close_engine().await?;
//     Ok(())
// }

// #[tokio::test]
// #[serial]
// async fn test_generate_transfer_proof() -> Result<()> {
//     let mut engine = DopEngine::new();
//     engine.start();
//     engine.wait_for_api_ready().await;

//     engine
//         .init_engine(
//             Some("database/test-transfer-proof.db"),
//             Some("Proof Engine"),
//             Some(false),
//             Some(true),  // useNativeArtifacts
//             Some(false),
//         )
//         .await?;

//     // ✅ Create real wallet
//     let mnemonic = engine.generate_mnemonic(Some(12)).await?;
//     let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

//     let wallet_info = engine
//         .create_wallet(&mnemonic, encryption_key, None)
//         .await?;

//     let wallet_id = wallet_info
//         .get("id")
//         .and_then(|v| v.as_str())
//         .expect("Missing wallet ID");
//         println!("Created Wallet: {:#?}", wallet_info);
//         let wallet_address = wallet_info
//         .get("dopAddress")
//         .and_then(|v| v.as_str())
//         .expect("Missing wallet address");

//     let payload = json!({
//         "network": "Polygon",
//         "walletId": wallet_id,
//         "encryptionKey": encryption_key,
//         "showSenderAddressToRecipient": true,
//         "memo": "Testing proof generation",
//         "tokenAmountRecipients": [
//             {
//                 "tokenAddress": "0x0000000000000000000000000000000000000001",
//                 "amount": "0x64",
//                 "recipientAddress": wallet_address
//             }
//         ],
//         "nftAmountRecipients": [],
//         "relayerFeeERC20AmountRecipient": {
//             "tokenAddress": "0x0000000000000000000000000000000000000001",
//             "amount": "0x10",
//             "recipientAddress": wallet_address
//         },
//         "sendWithPublicWallet": false,
//         "overallBatchMinGasPrice": "0x1000"
//     });

//     // ✅ Call the proof generator
//     let result = engine.generate_transfer_proof(payload).await?;
//     println!("Generated transfer proof: {:#?}", result);

//     assert!(result.is_object(), "Expected a JSON object as transfer proof result");

//     engine.close_engine().await?;
//     Ok(())
// }


// #[tokio::test]
// #[serial]
// async fn test_populate_proved_transfer() -> Result<()> {
//     let mut engine = DopEngine::new();
//     engine.start();
//     engine.wait_for_api_ready().await;
//     engine
//         .init_engine(
//             Some("database/test-populate-transfer.db"),
//             Some("Transfer Engine"),
//             Some(false),
//             Some(true),
//             Some(false),
//         )
//         .await?;

//     // Assumes generate_transfer_proof was called already
//     let payload = json!({
//         "network": "Polygon",
//         "walletId": "your_wallet_id",
//         "showSenderAddressToRecipient": true,
//         "memo": "Testing populate transfer",
//         "tokenAmountRecipients": [],
//         "nftAmountRecipients": [],
//         "relayerFeeERC20AmountRecipient": {},
//         "sendWithPublicWallet": false,
//         "overallBatchMinGasPrice": "0x1000",
//         "gasDetails": {
//             "evmGasType": "Type2",
//             "gasEstimate": "0x1234",
//             "maxFeePerGas": "0x4567",
//             "maxPriorityFeePerGas": "0x89"
//         }
//     });

//     let result = engine.populate_proved_transfer(payload).await?;
//     println!("Populated transaction: {:#?}", result);

//     assert!(result.get("transaction").is_some(), "Transaction field missing");

//     engine.close_engine().await?;
//     Ok(())
// }