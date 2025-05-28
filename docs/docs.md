# DOP Rust SDK Documentation

A comprehensive guide to using the DOP (Decentralized Operations Protocol) Rust SDK for blockchain wallet operations, encryption, decryption, and transaction management.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Components](#core-components)
- [Wallet Operations](#wallet-operations)
- [Transaction Encryption](#transaction-encryption)
- [Transaction Decryption](#transaction-decryption)
- [Balance Management](#balance-management)
- [Transfer Operations](#transfer-operations)
- [Error Handling](#error-handling)
- [Examples](#examples)
- [Testing](#testing)

## Installation

### Option 1: Git Dependency (Recommended)

Add to your `Cargo.toml`:

```toml
[dependencies]
dop = { git = "https://github.com/VAR-META-Tech/dop-rust", branch = "main" }
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
serde_json = "1.0"
```

### Option 2: Local Development

```bash
git clone https://github.com/VAR-META-Tech/dop-rust.git
cd dop-rust
cargo build
```

## Quick Start

```rust
use dop::dop::DopClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize client
    let mut client = DopClient::new();
    client.start();
    client.wait_for_api_ready().await;
    
    // Initialize engine
    client.init_engine(None, None, None, None, None).await?;
    
    // Your operations here...
    
    Ok(())
}
```

## Core Components

### DopClient

The main client for interacting with the DOP protocol:

```rust
use dop::dop::DopClient;

let mut client = DopClient::new();
client.start();                           // Start the Node.js backend
client.wait_for_api_ready().await;        // Wait for API to be ready
client.init_engine(None, None, None, None, None).await?;  // Initialize engine
```

### Provider Configuration

Configure blockchain providers for different networks:

```rust
use serde_json::json;

let fallback_providers = json!({
    "chainId": 11155111,  // Ethereum Sepolia
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

client.load_provider(fallback_providers, "Ethereum_Sepolia", Some(10_000)).await?;
```

## Wallet Operations

### Generate Mnemonic

```rust
// Generate a 12-word mnemonic
let mnemonic = client.generate_mnemonic(Some(12)).await?;
println!("Generated mnemonic: {}", mnemonic);
```

### Create Wallet

```rust
let mnemonic = client.generate_mnemonic(Some(12)).await?;
let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";

let wallet_info = client
    .create_wallet(&mnemonic, encryption_key, None)
    .await?;

let dop_wallet_id = wallet_info["dopAddress"].as_str().unwrap();
println!("Created wallet: {}", dop_wallet_id);
```

### Import Existing Wallet

```rust
let existing_mnemonic = "your twelve word mnemonic phrase goes here like this example";
let encryption_key = "your_encryption_key_here";

let wallet_info = client
    .create_wallet(existing_mnemonic, encryption_key, None)
    .await?;
```

## Transaction Encryption

### Encrypt Basic Transaction

```rust
use dop::dop::{DopERC20AmountRecipient, TransactionGasDetails};

let erc20_recipients = vec![
    DopERC20AmountRecipient {
        token_address: "0xTokenAddress".into(),
        amount: "1000".into(),
        recipient_address: "0xRecipientAddress".into(),
    }
];

let gas_details = TransactionGasDetails::Type0 {
    gas_estimate: "21000".into(),
    gas_price: "1000000000".into(),
};

let result = client
    .generate_encrypt_proof(
        "V2_PoseidonMerkle".into(),
        "Ethereum_Sepolia".into(),
        wallet_id,
        encryption_key,
        erc20_recipients,
        vec![], // NFT recipients
        None,   // Fee token details
        false,  // Use relayer
        None,   // Relayer fee
        gas_details,
    )
    .await?;
```

### Populate Encrypted Transaction

```rust
let tx = client
    .populate_proved_encrypt(
        "V2_PoseidonMerkle".into(),
        "Ethereum_Sepolia".into(),
        wallet_id,
        erc20_recipients,
        vec![], // NFT recipients
        None,   // Fee token details
        false,  // Use relayer
        None,   // Relayer fee
        gas_details,
    )
    .await?;

println!("Transaction to: {}", tx.transaction.to);
println!("Transaction data length: {}", tx.transaction.data.len());
```

## Transaction Decryption

### Generate Decrypt Proof

```rust
let result = client
    .generate_decrypt_proof(
        "V2_PoseidonMerkle".into(),
        "Ethereum_Sepolia".into(),
        wallet_id,
        encryption_key,
        vec![], // ERC20 recipients
        vec![], // NFT recipients
        None,   // Fee token details
        false,  // Use relayer
        None,   // Relayer fee
        "0".into(), // Min amount
    )
    .await?;
```

### Decrypt to Origin

```rust
let result = client
    .generate_decrypt_to_origin_proof(
        "original_txid".into(),
        "V2_PoseidonMerkle".into(),
        "Ethereum_Sepolia".into(),
        wallet_id,
        encryption_key,
        vec![], // ERC20 recipients
        vec![], // NFT recipients
        "0".into(), // Min amount
    )
    .await?;
```

### Decrypt Base Token

```rust
use dop::dop::DopERC20Amount;

let wrapped_amount = DopERC20Amount {
    token_address: "0xTokenAddress".into(),
    amount: "1000".into(),
};

let result = client
    .generate_decrypt_base_token_proof(
        "V2_PoseidonMerkle".into(),
        "Ethereum_Sepolia".into(),
        "0xPublicWallet".into(),
        wallet_id,
        encryption_key,
        wrapped_amount,
        None,   // Fee token details
        false,  // Use relayer
        None,   // Relayer fee
    )
    .await?;
```

## Balance Management

### Check Encrypted Balances

```rust
let balances = client
    .get_encrypted_balances(wallet_id, "Ethereum_Sepolia".into())
    .await?;

println!("Encrypted balances: {:?}", balances);
```

### Get Decrypted Balances

```rust
let balances = client
    .get_decrypted_balances(
        wallet_id,
        encryption_key,
        "Ethereum_Sepolia".into(),
    )
    .await?;

println!("Decrypted balances: {:?}", balances);
```

## Transfer Operations

### Direct Transfer

```rust
use dop::dop::TransferInfo;

let transfer_info = TransferInfo {
    recipient: "0xRecipientAddress".into(),
    amount: "1000".into(),
    token_address: Some("0xTokenAddress".into()),
};

let result = client
    .transfer(
        wallet_id,
        encryption_key,
        transfer_info,
        gas_details,
        "Ethereum_Sepolia".into(),
    )
    .await?;
```

## Error Handling

The SDK uses `anyhow::Result` for error handling. Common error patterns:

```rust
use anyhow::Result;

async fn example_with_error_handling() -> Result<()> {
    let client = DopClient::new();
    
    match client.generate_mnemonic(Some(12)).await {
        Ok(mnemonic) => {
            println!("✅ Generated mnemonic: {}", mnemonic);
        }
        Err(e) => {
            eprintln!("❌ Failed to generate mnemonic: {:?}", e);
            return Err(e);
        }
    }
    
    Ok(())
}
```

## Gas Estimation

### Estimate Gas for Unproven Operations

```rust
let gas_estimate = client
    .gas_estimate_for_unproven_decrypt(
        "V2_PoseidonMerkle".into(),
        "Ethereum_Sepolia".into(),
        wallet_id,
        encryption_key,
        vec![], // ERC20 recipients
        vec![], // NFT recipients
        gas_details,
        None,   // Fee token details
        false,  // Use relayer
        "0".into(), // Min amount
    )
    .await?;

println!("Estimated gas: {:?}", gas_estimate);
```

## Examples

### Complete Wallet Setup and Transaction

```rust
use dop::dop::{DopClient, DopERC20AmountRecipient, TransactionGasDetails};
use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Setup client
    let mut client = DopClient::new();
    client.start();
    client.wait_for_api_ready().await;
    client.init_engine(None, None, None, None, None).await?;
    
    // 2. Configure provider
    let fallback_providers = json!({
        "chainId": 11155111,
        "providers": [{
            "provider": "https://sepolia.drpc.org",
            "priority": 3,
            "weight": 3
        }]
    });
    
    client.load_provider(fallback_providers, "Ethereum_Sepolia", Some(10_000)).await?;
    
    // 3. Create wallet
    let mnemonic = client.generate_mnemonic(Some(12)).await?;
    let encryption_key = "0101010101010101010101010101010101010101010101010101010101010101";
    let wallet_info = client.create_wallet(&mnemonic, encryption_key, None).await?;
    let wallet_id = wallet_info["dopAddress"].as_str().unwrap();
    
    // 4. Prepare transaction
    let recipients = vec![DopERC20AmountRecipient {
        token_address: "0x1234...".into(),
        amount: "1000".into(),
        recipient_address: "0x5678...".into(),
    }];
    
    let gas_details = TransactionGasDetails::Type0 {
        gas_estimate: "21000".into(),
        gas_price: "1000000000".into(),
    };
    
    // 5. Generate and populate transaction
    let _proof = client.generate_encrypt_proof(
        "V2_PoseidonMerkle".into(),
        "Ethereum_Sepolia".into(),
        wallet_id.to_string(),
        encryption_key.to_string(),
        recipients.clone(),
        vec![],
        None,
        false,
        None,
        gas_details.clone(),
    ).await?;
    
    let tx = client.populate_proved_encrypt(
        "V2_PoseidonMerkle".into(),
        "Ethereum_Sepolia".into(),
        wallet_id.to_string(),
        recipients,
        vec![],
        None,
        false,
        None,
        gas_details,
    ).await?;
    
    println!("Transaction ready: {}", tx.transaction.to);
    Ok(())
}
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test wallet_tests
cargo test tx_encrypt_tests
cargo test tx_decrypt_tests
cargo test balance_tests
cargo test transfer_tests
```

### Test Structure

The SDK includes comprehensive tests in the [`tests/`](tests/) directory:

- [`wallet_tests.rs`](tests/wallet_tests.rs) - Wallet creation and management
- [`tx_encrypt_tests.rs`](tests/tx_encrypt_tests.rs) - Transaction encryption
- [`tx_decrypt_tests.rs`](tests/tx_decrypt_tests.rs) - Transaction decryption  
- [`balance_tests.rs`](tests/balance_tests.rs) - Balance operations
- [`transfer_tests.rs`](tests/transfer_tests.rs) - Transfer operations
- [`engine_tests.rs`](tests/engine_tests.rs) - Engine initialization
- [`callback_tests.rs`](tests/callback_tests.rs) - Callback handling

## Architecture

The DOP Rust SDK is a hybrid implementation:

- **Rust Layer**: Provides type-safe APIs and handles HTTP communication
- **TS Backend**: Implements the core cryptographic and blockchain logic
- **Automatic Build**: The [`build.rs`](build.rs) script automatically builds the Node.js backend

## Prerequisites

- Rust ≥ 1.70
- Node.js + npm
- Active internet connection for blockchain providers

## Support

For issues and questions:

1. Check the test files for usage examples
2. Review the [README.md](README.md) for setup instructions
3. Open an issue on the GitHub repository

---