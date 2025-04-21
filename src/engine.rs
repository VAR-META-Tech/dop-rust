// --- dop_engine.rs ---
// Unified DopEngine implementation using multiple impl blocks for modularity

use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;

pub struct DopEngine {
    child: Option<Child>,
    client: Client,
    port: u16,
}

// --- Constructor & Base Methods ---
impl DopEngine {
    pub fn new() -> Self {
        Self::with_port(3000)
    }

    pub fn with_port(port: u16) -> Self {
        Self {
            child: None,
            client: Client::new(),
            port,
        }
    }

    fn base_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    pub fn start(&mut self) {
        println!("Starting Node.js DOP Engine...");
        let child = Command::new("node")
            .arg("ts-lib/dist/index.js")
            .spawn()
            .expect("Failed to start Node Engine");
        self.child = Some(child);
    }

    pub fn stop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            if let Ok(Some(_)) = child.try_wait() {
                println!("Node.js process already exited.");
            } else {
                child.kill().expect("Failed to kill Node Engine");
                println!("Node.js process killed.");
            }
        }
    }
}

// --- Lifecycle Methods ---
impl DopEngine {
    pub async fn wait_for_api_ready(&self) {
        for _ in 0..10 {
            if self.client.get(&format!("{}/health", self.base_url())).send().await.is_ok() {
                println!("Node.js API is ready");
                return;
            }
            println!("Waiting for Node.js API...");
            sleep(Duration::from_secs(1)).await;
        }
        panic!("Node.js API failed to start");
    }

    pub async fn init_engine(
        &self,
        db_path: Option<&str>,
        engine_name: Option<&str>,
        should_debug: Option<bool>,
        use_native_artifacts: Option<bool>,
        skip_merkletree_scans: Option<bool>,
    ) -> Result<()> {
        let payload = json!({
            "dbPath": db_path.unwrap_or("database/DOP.db"),
            "engineName": engine_name.unwrap_or("DOP Engine"),
            "shouldDebug": should_debug.unwrap_or(false),
            "useNativeArtifacts": use_native_artifacts.unwrap_or(false),
            "skipMerkletreeScans": skip_merkletree_scans.unwrap_or(false),
        });

        self.client
            .post(&format!("{}/init", self.base_url()))
            .json(&payload)
            .send()
            .await
            .context("Failed to call /init")?;
        Ok(())
    }

    pub async fn close_engine(&self) -> Result<()> {
        self.client.get(&format!("{}/close", self.base_url())).send().await?;
        Ok(())
    }

    pub async fn get_engine_info(&self) -> Result<Value> {
        let res = self
            .client
            .get(&format!("{}/engine", self.base_url()))
            .send()
            .await?
            .json::<Value>()
            .await?;

        if res.get("wallets").is_none() {
            Err(anyhow::anyhow!("Invalid engine info response: wallets missing"))
        } else {
            Ok(res)
        }
    }
}

// --- Utility Methods ---
impl DopEngine {
    pub async fn set_loggers(&self) -> Result<()> {
        self.client
            .post(&format!("{}/set-loggers", self.base_url()))
            .send()
            .await?
            .error_for_status()
            .context("Failed to call /set-loggers")?;
        Ok(())
    }

    pub async fn load_provider(&self, config: Value, network: &str, polling_interval: u64) -> Result<Value> {
        let payload = json!({
            "config": config,
            "network": network,
            "pollingInterval": polling_interval
        });

        let res = self
            .client
            .post(&format!("{}/load-provider", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(res)
    }
}

// --- Wallet Methods ---
impl DopEngine {
    pub async fn generate_mnemonic(&self, words: Option<u8>) -> Result<String> {
        let url = match words {
            Some(24) => format!("{}/mnemonic?words=24", self.base_url()),
            _ => format!("{}/mnemonic", self.base_url()),
        };

        let res = self.client.get(&url).send().await?.json::<Value>().await?;
        res.get("mnemonic")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Mnemonic field missing or invalid in response"))
    }

    pub async fn create_wallet(&self, mnemonic: &str, encryption_key: &str, creation_block_numbers: Option<HashMap<&str, u64>>) -> Result<Value> {
        let mut payload = json!({
            "mnemonic": mnemonic,
            "encryptionKey": encryption_key,
        });

        if let Some(blocks) = creation_block_numbers {
            payload["creationBlockNumbers"] = json!(blocks);
        }

        let res = self
            .client
            .post(&format!("{}/wallet", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(res)
    }

    pub async fn get_wallet(&self, wallet_id: &str) -> Result<Value> {
        let res = self
            .client
            .get(&format!("{}/wallet/{}", self.base_url(), wallet_id))
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(res)
    }

    pub async fn get_wallet_mnemonic(&self, wallet_id: &str, encryption_key: &str) -> Result<String> {
        let res = self
            .client
            .get(&format!("{}/wallet/{}/mnemonic?encryptionKey={}", self.base_url(), wallet_id, encryption_key))
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("mnemonic")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing mnemonic in response"))
    }

    pub async fn get_shareable_viewing_key(&self, wallet_id: &str) -> Result<String> {
        let res = self
            .client
            .get(&format!("{}/wallet/{}/shareable-viewing-key", self.base_url(), wallet_id))
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("shareableViewingKey")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing shareableViewingKey in response"))
    }

    pub async fn create_view_only_wallet(&self, encryption_key: &str, shareable_viewing_key: &str, creation_block_numbers: Option<HashMap<&str, u64>>) -> Result<Value> {
        let mut payload = json!({
            "encryptionKey": encryption_key,
            "shareableViewingKey": shareable_viewing_key,
        });

        if let Some(blocks) = creation_block_numbers {
            payload["creationBlockNumbers"] = json!(blocks);
        }

        let res = self
            .client
            .post(&format!("{}/wallet/view-only", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(res)
    }

    pub async fn sign_message_with_wallet(&self, wallet_id: &str, message: &str) -> Result<String> {
        let payload = json!({ "walletId": wallet_id, "message": message });

        let res = self
            .client
            .post(&format!("{}/wallet/sign-message", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("signature")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing signature in response"))
    }

    pub async fn load_wallet_by_id(&self, encryption_key: &str, wallet_id: &str, is_view_only: bool) -> Result<Value> {
        let payload = json!({
            "encryptionKey": encryption_key,
            "dopWalletID": wallet_id,
            "isViewOnlyWallet": is_view_only
        });

        let res = self
            .client
            .post(&format!("{}/wallet/load", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(res)
    }
}

// --- Transfer Methods ---
impl DopEngine {
    pub async fn gas_estimate_for_unproven_transfer(&self, txid_version: &str, network: &str, wallet_id: &str, encryption_key: &str, memo: &str, token_amount_recipients: Vec<Value>, nft_amount_recipients: Vec<Value>, tx_gas_details: Value, fee_token_details: Value, send_with_public_wallet: bool) -> Result<Value> {
        let payload = json!({
            "txidVersion": txid_version,
            "network": network,
            "walletId": wallet_id,
            "encryptionKey": encryption_key,
            "memoText": memo,
            "erc20AmountRecipients": token_amount_recipients,
            "nftAmountRecipients": nft_amount_recipients,
            "transactionGasDetailsSerialized": tx_gas_details,
            "feeTokenDetails": fee_token_details,
            "sendWithPublicWallet": send_with_public_wallet
        });

        let res = self
            .client
            .post(&format!("{}/gas-estimate-unproven", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(res)
    }

    pub async fn generate_transfer_proof(&self, payload: Value) -> Result<Value> {
        let res = self
            .client
            .post(&format!("{}/generate-transfer-proof", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(res)
    }

    pub async fn populate_proved_transfer(&self, payload: Value) -> Result<Value> {
        let res = self
            .client
            .post(&format!("{}/populate-transfer", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .error_for_status()? // ensure HTTP status is ok
            .json::<Value>()
            .await?;
        Ok(res)
    }
}

impl Drop for DopEngine {
    fn drop(&mut self) {
        self.stop();
    }
}