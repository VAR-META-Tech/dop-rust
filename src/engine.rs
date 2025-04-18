use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;
use serde_json::json;
pub struct DopEngine {
    child: Option<Child>,
    client: Client,
    port: u16,
}

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

    pub fn start(&mut self) {
        println!("Starting Node.js DOP Engine...");
        let child = Command::new("node")
            .arg("ts-lib/dist/index.js")
            .spawn()
            .expect("Failed to start Node Engine");

        self.child = Some(child);
    }

    fn base_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    pub async fn wait_for_api_ready(&self) {
        for _ in 0..10 {
            if self
                .client
                .get(&format!("{}/health", self.base_url()))
                .send()
                .await
                .is_ok()
            {
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
            "dbPath": db_path.unwrap_or("DOP.db"),
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
    

    pub async fn engine_status(&self) -> Result<String> {
        let res = self
            .client
            .get(&format!("{}/status", self.base_url()))
            .send()
            .await?
            .json::<Value>()
            .await?;
    
        res.get("status")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| anyhow::anyhow!("Missing status in response"))
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

    pub async fn close_engine(&self) -> Result<()> {
        self.client
            .get(&format!("{}/close", self.base_url()))
            .send()
            .await?;
        Ok(())
    }

    pub async fn generate_mnemonic(&self, words: Option<u8>) -> Result<String> {
        let url = match words {
            Some(24) => format!("{}/mnemonic?words=24", self.base_url()),
            _ => format!("{}/mnemonic", self.base_url()), // default is 12
        };

        let res = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("mnemonic")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Mnemonic field missing or invalid in response"))
    }

    pub async fn create_wallet(
        &self,
        mnemonic: &str,
        encryption_key: &str,
        creation_block_numbers: Option<HashMap<&str, u64>>,
    ) -> Result<Value> {
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

impl Drop for DopEngine {
    fn drop(&mut self) {
        self.stop(); // Just stop the process
    }
}


