# DOP Rust SDK Tutorial

A comprehensive guide with practical examples for using the DOP (Decentralized Operations Protocol) Rust SDK for blockchain wallet operations, encryption, decryption, and transaction management.

This document demonstrates all major features of the SDK with working code examples that you can run and modify.

## Setup and Installation

First, let's set up the necessary dependencies in your `Cargo.toml` file and import the required modules.

```toml
# Add these dependencies to your Cargo.toml:
[dependencies]
dop = { git = "https://github.com/VAR-META-Tech/dop-rust", branch = "main" }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
serde_json = "1.0"
```

```rust
// Required imports for the DOP SDK
use dop::dop::{
    DopClient, 
    DopERC20AmountRecipient, 
    DopERC20Amount,
    TransactionGasDetails,
    TransferInfo
};
use anyhow::Result;
use serde_json::json;

println!("✅ Dependencies imported successfully!");
```

## Client Initialization

Initialize the DOP client, start the Node.js backend, and prepare it for operations.

```rust
async fn initialize_client() -> Result<DopClient> {
    println!("🚀 Initializing DOP client...");
    
    // Create new client instance
    let mut client = DopClient::new();
    
    // Start the Node.js backend
    client.start();
    println!("⏳ Starting Node.js backend...");
    
    // Wait for API to be ready
    client.wait_for_api_ready().await;
    println!("✅ API is ready!");
    
    // Initialize the engine with default parameters
    client.init_engine(None, None, None, None, None).await?;
    println!("✅ Engine initialized successfully!");
    
    Ok(client)
}

// Example usage (uncomment to run):
// let client = initialize_client().await?;
println!("Client initialization function ready!");
```

## Provider Configuration

Configure blockchain providers for different networks. This example shows how to set up Ethereum Sepolia testnet providers.

```rust
async fn configure_provider(client: &mut DopClient) -> Result<()> {
    println!("🔗 Configuring blockchain provider...");
    
    // Configure Ethereum Sepolia testnet providers
    let fallback_providers = json!({
        "chainId": 11155111,  // Ethereum Sepolia chain ID
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
            },
            {
                "provider": "https://rpc.sepolia.org",
                "priority": 2,
                "weight": 1
            }
        ]
    });

    // Load the provider configuration
    client.load_provider(
        fallback_providers, 
        "Ethereum_Sepolia", 
        Some(10_000)  // Timeout in milliseconds
    ).await?;
    
    println!("✅ Provider configured successfully for Ethereum Sepolia!");
    Ok(())
}

// Alternative provider configurations for other networks
fn get_mainnet_config() -> serde_json::Value {
    json!({
        "chainId": 1,
        "providers": [
            {
                "provider": "https://ethereum.drpc.org",
                "priority": 3,
                "weight": 3
            }
        ]
    })
}

println!("Provider configuration functions ready!");
```

## Mnemonic Generation

Generate secure mnemonic phrases for wallet creation with different word counts.

```rust
async fn generate_mnemonics(client: &DopClient) -> Result<()> {
    println!("🔑 Generating mnemonic phrases...");
    
    // Generate 12-word mnemonic (most common)
    let mnemonic_12 = client.generate_mnemonic(Some(12)).await?;
    println!("📝 12-word mnemonic: {}", mnemonic_12);
    
    // Generate 24-word mnemonic (more secure)
    let mnemonic_24 = client.generate_mnemonic(Some(24)).await?;
    println!("📝 24-word mnemonic: {}", mnemonic_24);
    
    // Generate default mnemonic (usually 12 words)
    let mnemonic_default = client.generate_mnemonic(None).await?;
    println!("📝 Default mnemonic: {}", mnemonic_default);
    
    println!("✅ Mnemonics generated successfully!");
    Ok(())
}

// Validate mnemonic format
fn validate_mnemonic(mnemonic: &str) -> bool {
    let words: Vec<&str> = mnemonic.split_whitespace().collect();
    matches!(words.len(), 12 | 15 | 18 | 21 | 24)
}

println!("Mnemonic generation functions ready!");
```

## Wallet Creation and Management

Create new wallets and import existing wallets using mnemonic phrases and encryption keys.

```rust
async fn create_new_wallet(client: &DopClient) -> Result<(String, String, String)> {
    println!("👛 Creating new wallet...");
    
    // Generate mnemonic for new wallet
    let mnemonic = client.generate_mnemonic(Some(12)).await?;
    println!("Generated mnemonic: {}", mnemonic);
    
    // Use a secure encryption key (64 hex characters)
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";
    
    // Create the wallet
    let wallet_info = client
        .create_wallet(&mnemonic, encryption_key, None)
        .await?;
    
    // Extract wallet ID (DOP address)
    let dop_wallet_id = wallet_info["dopAddress"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to get dopAddress from wallet info"))?
        .to_string();
    
    println!("✅ Wallet created successfully!");
    println!("DOP Wallet ID: {}", dop_wallet_id);
    
    Ok((mnemonic, encryption_key.to_string(), dop_wallet_id))
}

async fn import_existing_wallet(client: &DopClient, mnemonic: &str, encryption_key: &str) -> Result<String> {
    println!("📥 Importing existing wallet...");
    
    // Import wallet using existing mnemonic
    let wallet_info = client
        .create_wallet(mnemonic, encryption_key, None)
        .await?;
    
    let dop_wallet_id = wallet_info["dopAddress"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to get dopAddress from wallet info"))?
        .to_string();
    
    println!("✅ Wallet imported successfully!");
    println!("DOP Wallet ID: {}", dop_wallet_id);
    
    Ok(dop_wallet_id)
}

// Example of secure key generation (you should use a proper key derivation function)
fn generate_encryption_key() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now().hash(&mut hasher);
    format!("{:016x}{:016x}{:016x}{:016x}", 
            hasher.finish(), hasher.finish(), hasher.finish(), hasher.finish())
}

println!("Wallet management functions ready!");
```

## Transaction Encryption

Generate encryption proofs and populate encrypted transactions for ERC20 tokens with various gas configurations.

```rust
async fn encrypt_transaction_example(
    client: &DopClient,
    wallet_id: &str,
    encryption_key: &str
) -> Result<()> {
    println!("🔐 Encrypting transaction...");
    
    // Define ERC20 recipients
    let erc20_recipients = vec![
        DopERC20AmountRecipient {
            token_address: "0x1234567890123456789012345678901234567890".into(),
            amount: "1000000000000000000".into(), // 1 token with 18 decimals
            recipient_address: "0x9876543210987654321098765432109876543210".into(),
        },
        DopERC20AmountRecipient {
            token_address: "0x2345678901234567890123456789012345678901".into(),
            amount: "500000000000000000".into(), // 0.5 tokens with 18 decimals
            recipient_address: "0x8765432109876543210987654321098765432109".into(),
        }
    ];
    
    // Configure gas details (Type 0 transaction)
    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "150000".into(),
        gas_price: "20000000000".into(), // 20 Gwei
    };
    
    // Generate encryption proof
    println!("Generating encryption proof...");
    let proof_result = client
        .generate_encrypt_proof(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            erc20_recipients.clone(),
            vec![], // No NFT recipients
            None,   // No fee token details
            false,  // Don't use relayer
            None,   // No relayer fee
            gas_details.clone(),
        )
        .await?;
    
    println!("✅ Encryption proof generated successfully!");
    
    // Populate the encrypted transaction
    println!("Populating encrypted transaction...");
    let tx = client
        .populate_proved_encrypt(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            erc20_recipients,
            vec![], // No NFT recipients
            None,   // No fee token details
            false,  // Don't use relayer
            None,   // No relayer fee
            gas_details,
        )
        .await?;
    
    println!("✅ Transaction populated successfully!");
    println!("Transaction to: {}", tx.transaction.to);
    println!("Transaction data length: {} bytes", tx.transaction.data.len());
    
    Ok(())
}

// Example with EIP-1559 gas pricing
fn create_eip1559_gas_details() -> TransactionGasDetails {
    TransactionGasDetails::Type2 {
        gas_estimate: "150000".into(),
        max_fee_per_gas: "30000000000".into(),      // 30 Gwei
        max_priority_fee_per_gas: "2000000000".into(), // 2 Gwei
    }
}

println!("Transaction encryption functions ready!");
```

## Transaction Decryption

Generate decrypt proofs, decrypt to origin, and decrypt base tokens with proper recipient configurations.

```rust
async fn decrypt_transaction_examples(
    client: &DopClient,
    wallet_id: &str,
    encryption_key: &str
) -> Result<()> {
    println!("🔓 Decrypting transactions...");
    
    // Example 1: Basic decrypt proof
    println!("1. Generating basic decrypt proof...");
    let decrypt_result = client
        .generate_decrypt_proof(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            vec![], // ERC20 recipients
            vec![], // NFT recipients
            None,   // Fee token details
            false,  // Use relayer
            None,   // Relayer fee
            "0".into(), // Min amount
        )
        .await?;
    
    println!("✅ Basic decrypt proof generated!");
    
    // Example 2: Decrypt to origin
    println!("2. Generating decrypt to origin proof...");
    let original_txid = "0x1234567890123456789012345678901234567890123456789012345678901234";
    let decrypt_origin_result = client
        .generate_decrypt_to_origin_proof(
            original_txid.into(),
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            vec![], // ERC20 recipients
            vec![], // NFT recipients
            "0".into(), // Min amount
        )
        .await?;
    
    println!("✅ Decrypt to origin proof generated!");
    
    // Example 3: Decrypt base token
    println!("3. Generating decrypt base token proof...");
    let wrapped_amount = DopERC20Amount {
        token_address: "0x1234567890123456789012345678901234567890".into(),
        amount: "1000000000000000000".into(), // 1 token
    };
    
    let public_wallet = "0x9876543210987654321098765432109876543210";
    
    let decrypt_base_result = client
        .generate_decrypt_base_token_proof(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            public_wallet.into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            wrapped_amount,
            None,   // Fee token details
            false,  // Use relayer
            None,   // Relayer fee
        )
        .await?;
    
    println!("✅ Decrypt base token proof generated!");
    
    Ok(())
}

// Example: Decrypt with specific ERC20 recipients
async fn decrypt_with_recipients(
    client: &DopClient,
    wallet_id: &str,
    encryption_key: &str
) -> Result<()> {
    let erc20_recipients = vec![
        DopERC20AmountRecipient {
            token_address: "0x1234567890123456789012345678901234567890".into(),
            amount: "500000000000000000".into(),
            recipient_address: "0x9876543210987654321098765432109876543210".into(),
        }
    ];
    
    let result = client
        .generate_decrypt_proof(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            erc20_recipients,
            vec![], // NFT recipients
            None,   // Fee token details
            false,  // Use relayer
            None,   // Relayer fee
            "100000000000000000".into(), // Min amount: 0.1 tokens
        )
        .await?;
    
    println!("✅ Decrypt with recipients completed!");
    Ok(())
}

println!("Transaction decryption functions ready!");
```

## Balance Operations

Check encrypted balances and get decrypted balances for wallets on different networks.

```rust
async fn balance_operations(
    client: &DopClient,
    wallet_id: &str,
    encryption_key: &str
) -> Result<()> {
    println!("💰 Checking wallet balances...");
    
    // Get encrypted balances
    println!("1. Fetching encrypted balances...");
    let encrypted_balances = client
        .get_encrypted_balances(wallet_id, "Ethereum_Sepolia".into())
        .await?;
    
    println!("📊 Encrypted balances:");
    println!("{:#}", serde_json::to_string_pretty(&encrypted_balances)?);
    
    // Get decrypted balances
    println!("2. Fetching decrypted balances...");
    let decrypted_balances = client
        .get_decrypted_balances(
            wallet_id,
            encryption_key,
            "Ethereum_Sepolia".into(),
        )
        .await?;
    
    println!("💎 Decrypted balances:");
    println!("{:#}", serde_json::to_string_pretty(&decrypted_balances)?);
    
    // Process balance data
    if let Some(balances) = decrypted_balances.as_object() {
        for (token, balance) in balances {
            if let Some(balance_str) = balance.as_str() {
                println!("Token {}: {} units", token, balance_str);
            }
        }
    }
    
    println!("✅ Balance operations completed!");
    Ok(())
}

// Helper function to format balance with decimals
fn format_token_balance(balance: &str, decimals: u8) -> Result<String> {
    let balance_num: u128 = balance.parse()?;
    let divisor = 10_u128.pow(decimals as u32);
    let whole = balance_num / divisor;
    let fraction = balance_num % divisor;
    
    Ok(format!("{}.{:0width$}", whole, fraction, width = decimals as usize))
}

// Example usage
fn example_balance_formatting() {
    let balance = "1500000000000000000"; // 1.5 tokens with 18 decimals
    match format_token_balance(balance, 18) {
        Ok(formatted) => println!("Formatted balance: {} tokens", formatted),
        Err(e) => println!("Error formatting balance: {:?}", e),
    }
}

println!("Balance operation functions ready!");
```

## Transfer Operations

Perform direct transfers with proper transfer info structures and handle different token types.

```rust
async fn transfer_operations(
    client: &DopClient,
    wallet_id: &str,
    encryption_key: &str
) -> Result<()> {
    println!("💸 Performing transfer operations...");
    
    // Example 1: ERC20 token transfer
    println!("1. Preparing ERC20 token transfer...");
    let erc20_transfer = TransferInfo {
        recipient: "0x9876543210987654321098765432109876543210".into(),
        amount: "1000000000000000000".into(), // 1 token with 18 decimals
        token_address: Some("0x1234567890123456789012345678901234567890".into()),
    };
    
    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "100000".into(),
        gas_price: "20000000000".into(), // 20 Gwei
    };
    
    println!("Executing ERC20 transfer...");
    let erc20_result = client
        .transfer(
            wallet_id,
            encryption_key,
            erc20_transfer,
            gas_details.clone(),
            "Ethereum_Sepolia".into(),
        )
        .await?;
    
    println!("✅ ERC20 transfer completed!");
    
    // Example 2: Native ETH transfer
    println!("2. Preparing native ETH transfer...");
    let eth_transfer = TransferInfo {
        recipient: "0x8765432109876543210987654321098765432109".into(),
        amount: "500000000000000000".into(), // 0.5 ETH
        token_address: None, // None for native token
    };
    
    let eth_gas_details = TransactionGasDetails::Type2 {
        gas_estimate: "21000".into(),
        max_fee_per_gas: "30000000000".into(),
        max_priority_fee_per_gas: "2000000000".into(),
    };
    
    println!("Executing ETH transfer...");
    let eth_result = client
        .transfer(
            wallet_id,
            encryption_key,
            eth_transfer,
            eth_gas_details,
            "Ethereum_Sepolia".into(),
        )
        .await?;
    
    println!("✅ ETH transfer completed!");
    
    Ok(())
}

// Helper function to create transfer info for different scenarios
fn create_transfer_info(
    recipient: &str,
    amount: &str,
    token_address: Option<&str>
) -> TransferInfo {
    TransferInfo {
        recipient: recipient.into(),
        amount: amount.into(),
        token_address: token_address.map(|addr| addr.into()),
    }
}

// Example: Batch transfer preparation
fn prepare_batch_transfers() -> Vec<TransferInfo> {
    vec![
        create_transfer_info(
            "0x1111111111111111111111111111111111111111",
            "1000000000000000000",
            Some("0x2222222222222222222222222222222222222222")
        ),
        create_transfer_info(
            "0x3333333333333333333333333333333333333333",
            "2000000000000000000",
            Some("0x4444444444444444444444444444444444444444")
        ),
        create_transfer_info(
            "0x5555555555555555555555555555555555555555",
            "100000000000000000", // 0.1 ETH
            None // Native token
        ),
    ]
}

println!("Transfer operation functions ready!");
```

## Gas Estimation

Estimate gas costs for unproven operations and handle different gas detail configurations.

```rust
async fn gas_estimation_examples(
    client: &DopClient,
    wallet_id: &str,
    encryption_key: &str
) -> Result<()> {
    println!("⛽ Estimating gas costs...");
    
    // Prepare sample recipients for estimation
    let erc20_recipients = vec![
        DopERC20AmountRecipient {
            token_address: "0x1234567890123456789012345678901234567890".into(),
            amount: "1000000000000000000".into(),
            recipient_address: "0x9876543210987654321098765432109876543210".into(),
        }
    ];
    
    // Initial gas details for estimation
    let initial_gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "200000".into(), // Conservative estimate
        gas_price: "20000000000".into(),
    };
    
    // Estimate gas for unproven decrypt operation
    println!("1. Estimating gas for unproven decrypt...");
    let decrypt_gas_estimate = client
        .gas_estimate_for_unproven_decrypt(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            erc20_recipients.clone(),
            vec![], // NFT recipients
            initial_gas_details.clone(),
            None,   // Fee token details
            false,  // Use relayer
            "0".into(), // Min amount
        )
        .await?;
    
    println!("📊 Decrypt gas estimate: {:?}", decrypt_gas_estimate);
    
    // Estimate gas for unproven encrypt operation
    println!("2. Estimating gas for unproven encrypt...");
    let encrypt_gas_estimate = client
        .gas_estimate_for_unproven_encrypt(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            erc20_recipients,
            vec![], // NFT recipients
            initial_gas_details,
            None,   // Fee token details
            false,  // Use relayer
            None,   // Relayer fee
        )
        .await?;
    
    println!("📊 Encrypt gas estimate: {:?}", encrypt_gas_estimate);
    
    println!("✅ Gas estimation completed!");
    Ok(())
}

// Helper function to optimize gas settings based on network conditions
fn optimize_gas_settings(base_fee: u64, priority_fee: u64, gas_limit: u64) -> TransactionGasDetails {
    // Calculate optimal fees based on network conditions
    let max_priority_fee = std::cmp::max(priority_fee, 1_000_000_000); // Min 1 Gwei
    let max_fee = base_fee * 2 + max_priority_fee; // 2x base fee + priority
    
    TransactionGasDetails::Type2 {
        gas_estimate: gas_limit.to_string(),
        max_fee_per_gas: max_fee.to_string(),
        max_priority_fee_per_gas: max_priority_fee.to_string(),
    }
}

// Example: Dynamic gas pricing
fn create_dynamic_gas_pricing(network_congestion: f64) -> TransactionGasDetails {
    let base_gas_price = 20_000_000_000u64; // 20 Gwei
    let adjusted_price = (base_gas_price as f64 * (1.0 + network_congestion)) as u64;
    
    TransactionGasDetails::Type0 {
        gas_estimate: "150000".into(),
        gas_price: adjusted_price.to_string(),
    }
}

println!("Gas estimation functions ready!");
```

## Error Handling Examples

Implement proper error handling patterns using anyhow::Result and handle common failure scenarios.

```rust
use anyhow::{Context, bail};

async fn error_handling_examples(client: &DopClient) -> Result<()> {
    println!("🛠️ Demonstrating error handling patterns...");
    
    // Example 1: Graceful error handling with context
    async fn safe_mnemonic_generation(client: &DopClient) -> Result<String> {
        client
            .generate_mnemonic(Some(12))
            .await
            .context("Failed to generate 12-word mnemonic")
    }
    
    // Example 2: Validation with custom errors
    fn validate_wallet_inputs(mnemonic: &str, encryption_key: &str) -> Result<()> {
        // Validate mnemonic word count
        let word_count = mnemonic.split_whitespace().count();
        if !matches!(word_count, 12 | 24) {
            bail!("Invalid mnemonic: expected 12 or 24 words, got {}", word_count);
        }
        
        // Validate encryption key format
        if encryption_key.len() != 64 {
            bail!("Invalid encryption key: expected 64 hex characters, got {}", encryption_key.len());
        }
        
        // Validate hex format
        if !encryption_key.chars().all(|c| c.is_ascii_hexdigit()) {
            bail!("Invalid encryption key: contains non-hexadecimal characters");
        }
        
        Ok(())
    }
    
    // Example 3: Retry logic for network operations
    async fn retry_operation<F, T>(
        operation: F,
        max_retries: usize,
        operation_name: &str,
    ) -> Result<T>
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>,
    {
        let mut last_error = None;
        
        for attempt in 1..=max_retries {
            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        println!("✅ {} succeeded on attempt {}", operation_name, attempt);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    println!("❌ {} failed on attempt {}: {:?}", operation_name, attempt, e);
                    last_error = Some(e);
                    
                    if attempt < max_retries {
                        println!("🔄 Retrying in 1 second...");
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Operation failed after {} attempts", max_retries)))
    }
    
    // Example 4: Comprehensive error handling for wallet creation
    async fn robust_wallet_creation(client: &DopClient) -> Result<(String, String, String)> {
        // Generate mnemonic with retry logic
        let mnemonic = retry_operation(
            || Box::pin(safe_mnemonic_generation(client)),
            3,
            "mnemonic generation",
        ).await?;
        
        let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";
        
        // Validate inputs
        validate_wallet_inputs(&mnemonic, encryption_key)
            .context("Wallet input validation failed")?;
        
        // Create wallet with proper error context
        let wallet_info = client
            .create_wallet(&mnemonic, encryption_key, None)
            .await
            .context("Failed to create wallet with generated mnemonic")?;
        
        // Extract wallet ID with error handling
        let dop_wallet_id = wallet_info
            .get("dopAddress")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid dopAddress in wallet response"))?
            .to_string();
        
        println!("✅ Robust wallet creation completed successfully!");
        Ok((mnemonic, encryption_key.to_string(), dop_wallet_id))
    }
    
    // Demonstrate error scenarios
    println!("Testing error scenarios...");
    
    // Test invalid mnemonic
    if let Err(e) = validate_wallet_inputs("invalid mnemonic", "0101010101010101010101010101010101010101010101010101010101010101") {
        println!("Expected error for invalid mnemonic: {:?}", e);
    }
    
    // Test invalid encryption key
    if let Err(e) = validate_wallet_inputs("word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12", "invalid_key") {
        println!("Expected error for invalid key: {:?}", e);
    }
    
    println!("✅ Error handling examples completed!");
    Ok(())
}

// Custom error types for specific scenarios
#[derive(Debug, thiserror::Error)]
enum DopSdkError {
    #[error("Invalid wallet configuration: {message}")]
    InvalidWalletConfig { message: String },
    
    #[error("Network operation failed after {attempts} attempts")]
    NetworkOperationFailed { attempts: usize },
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: String, available: String },
    
    #[error("Gas estimation failed: {reason}")]
    GasEstimationFailed { reason: String },
}

println!("Error handling functions ready!");
```

## Complete Transaction Workflow

Demonstrate a complete end-to-end transaction workflow combining all the previous examples into a working application.

```rust
#[tokio::main]
async fn complete_transaction_workflow() -> Result<()> {
    println!("🚀 Starting complete DOP SDK workflow...");
    println!("=====================================");
    
    // Step 1: Initialize client
    println!("\n📋 Step 1: Initialize DOP Client");
    let mut client = DopClient::new();
    client.start();
    client.wait_for_api_ready().await;
    client.init_engine(None, None, None, None, None).await
        .context("Failed to initialize DOP engine")?;
    
    // Step 2: Configure provider
    println!("\n📋 Step 2: Configure Blockchain Provider");
    let fallback_providers = json!({
        "chainId": 11155111,
        "providers": [{
            "provider": "https://sepolia.drpc.org",
            "priority": 3,
            "weight": 3,
            "maxLogsPerBatch": 2,
            "stallTimeout": 2500
        }]
    });
    
    client.load_provider(fallback_providers, "Ethereum_Sepolia", Some(10_000)).await
        .context("Failed to load blockchain provider")?;
    
    // Step 3: Create wallet
    println!("\n📋 Step 3: Create Wallet");
    let mnemonic = client.generate_mnemonic(Some(12)).await
        .context("Failed to generate mnemonic")?;
    println!("🔑 Generated mnemonic: {}", mnemonic);
    
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";
    let wallet_info = client.create_wallet(&mnemonic, encryption_key, None).await
        .context("Failed to create wallet")?;
    
    let wallet_id = wallet_info["dopAddress"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to extract wallet ID"))?;
    println!("👛 Wallet created: {}", wallet_id);
    
    // Step 4: Check initial balances
    println!("\n📋 Step 4: Check Initial Balances");
    let initial_balances = client
        .get_decrypted_balances(wallet_id, encryption_key, "Ethereum_Sepolia".into())
        .await
        .context("Failed to get initial balances")?;
    println!("💰 Initial balances: {}", serde_json::to_string_pretty(&initial_balances)?);
    
    // Step 5: Prepare transaction recipients
    println!("\n📋 Step 5: Prepare Transaction");
    let recipients = vec![
        DopERC20AmountRecipient {
            token_address: "0x1234567890123456789012345678901234567890".into(),
            amount: "1000000000000000000".into(), // 1 token
            recipient_address: "0x9876543210987654321098765432109876543210".into(),
        }
    ];
    
    // Step 6: Gas estimation
    println!("\n📋 Step 6: Estimate Gas");
    let initial_gas = TransactionGasDetails::Type0 {
        gas_estimate: "200000".into(),
        gas_price: "20000000000".into(),
    };
    
    let gas_estimate = client
        .gas_estimate_for_unproven_encrypt(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            recipients.clone(),
            vec![],
            initial_gas.clone(),
            None,
            false,
            None,
        )
        .await
        .context("Failed to estimate gas")?;
    
    println!("⛽ Gas estimate: {:?}", gas_estimate);
    
    // Step 7: Generate encryption proof
    println!("\n📋 Step 7: Generate Encryption Proof");
    let _proof_result = client
        .generate_encrypt_proof(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            recipients.clone(),
            vec![],
            None,
            false,
            None,
            initial_gas.clone(),
        )
        .await
        .context("Failed to generate encryption proof")?;
    
    println!("🔐 Encryption proof generated successfully!");
    
    // Step 8: Populate transaction
    println!("\n📋 Step 8: Populate Transaction");
    let tx = client
        .populate_proved_encrypt(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            recipients,
            vec![],
            None,
            false,
            None,
            initial_gas,
        )
        .await
        .context("Failed to populate transaction")?;
    
    println!("📝 Transaction populated:");
    println!("   To: {}", tx.transaction.to);
    println!("   Data length: {} bytes", tx.transaction.data.len());
    println!("   Gas limit: {}", tx.transaction.gas_limit);
    
    // Step 9: Check final balances (simulated)
    println!("\n📋 Step 9: Transaction Ready for Submission");
    println!("💡 In a real scenario, you would now:");
    println!("   1. Sign the transaction with your wallet");
    println!("   2. Submit it to the blockchain");
    println!("   3. Wait for confirmation");
    println!("   4. Check updated balances");
    
    // Step 10: Demonstrate decryption workflow
    println!("\n📋 Step 10: Demonstrate Decryption (for received funds)");
    let _decrypt_proof = client
        .generate_decrypt_proof(
            "V2_PoseidonMerkle".into(),
            "Ethereum_Sepolia".into(),
            wallet_id.to_string(),
            encryption_key.to_string(),
            vec![],
            vec![],
            None,
            false,
            None,
            "0".into(),
        )
        .await
        .context("Failed to generate decrypt proof")?;
    
    println!("🔓 Decrypt proof generated for receiving funds!");
    
    println!("\n🎉 Complete workflow finished successfully!");
    println!("=====================================");
    
    // Summary
    println!("\n📊 Workflow Summary:");
    println!("✅ Client initialized and configured");
    println!("✅ Wallet created: {}", wallet_id);
    println!("✅ Provider configured for Ethereum Sepolia");
    println!("✅ Gas estimation completed");
    println!("✅ Encryption proof generated");
    println!("✅ Transaction populated and ready");
    println!("✅ Decryption proof generated");
    
    Ok(())
}

// Helper function to run the complete workflow with error recovery
async fn run_workflow_with_recovery() -> Result<()> {
    match complete_transaction_workflow().await {
        Ok(_) => {
            println!("🎯 Workflow completed successfully!");
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Workflow failed: {:?}", e);
            eprintln!("💡 Check your network connection and try again");
            Err(e)
        }
    }
}

println!("Complete workflow function ready!");
println!("\n🎯 To run the complete workflow, call: complete_transaction_workflow().await");
```

## Summary and Next Steps

Congratulations! You've learned how to use the DOP Rust SDK with comprehensive examples covering:

### ✅ What We Covered

1. **Setup and Installation** - Adding dependencies and imports
2. **Client Initialization** - Starting the DOP client and backend
3. **Provider Configuration** - Setting up blockchain network providers
4. **Wallet Management** - Creating and importing wallets with mnemonics
5. **Transaction Encryption** - Encrypting transactions for privacy
6. **Transaction Decryption** - Decrypting received transactions
7. **Balance Operations** - Checking encrypted and decrypted balances
8. **Transfer Operations** - Performing ERC20 and native token transfers
9. **Gas Estimation** - Optimizing transaction costs
10. **Error Handling** - Robust error management patterns
11. **Complete Workflow** - End-to-end transaction processing

### 🚀 Next Steps

1. **Integration**: Integrate these examples into your own Rust application
2. **Testing**: Use the Ethereum Sepolia testnet for safe testing
3. **Customization**: Adapt the examples for your specific use case
4. **Production**: Configure mainnet providers for production deployment

### 🛠️ Key Tips

- Always use testnet for development and testing
- Implement proper error handling for production applications
- Store encryption keys securely (never hardcode them)
- Monitor gas prices for optimal transaction costs
- Test all operations thoroughly before mainnet deployment

### 📚 Additional Resources

- [DOP SDK Documentation](docs.md)
- [GitHub Repository](https://github.com/VAR-META-Tech/dop-rust)
- [Test Examples](tests/)

Happy building with DOP! 🎉