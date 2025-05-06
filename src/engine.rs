use reqwest::Client;
use serde_json::Value;
use std::arch::aarch64::uint64x1_t;
use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;

pub struct DopEngine {
    child: Option<Child>,
    client: Client,
}

impl DopEngine {
    pub fn new() -> Self {
        Self {
            child: None,
            client: Client::new(),
        }
    }

    pub fn start(&mut self) {
        let child = Command::new("node")
            .arg("ts-lib/dist/index.js")
            .spawn()
            .expect("Failed to start Node Engine");

        self.child = Some(child);
    }

    pub async fn wait_for_api_ready(&self) {
        for _ in 0..10 {
            if self
                .client
                .get("http://localhost:3000/health")
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

    pub async fn init_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.get("http://localhost:3000/init").send().await?;
        Ok(())
    }

    pub async fn engine_status(&self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get("http://localhost:3000/status")
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }

    pub async fn get_engine_info(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get("http://localhost:3000/engine")
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(res)
    }

    pub async fn close_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .get("http://localhost:3000/close")
            .send()
            .await?;
        Ok(())
    }

    pub async fn create_wallet(
        &self,
        mnemonic: &str,
        encryption_key: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post("http://localhost:3000/wallet")
            .json(&serde_json::json!({
                "mnemonic": mnemonic,
                "encryptionKey": encryption_key,
                "creationBlockNumbers": {
                    "Ethereum": 0,
                    "Polygon": 2
                }
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(res)
    }

    pub async fn get_wallet(&self, wallet_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(&format!("http://localhost:3000/wallet/{}", wallet_id))
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(res)
    }

    pub fn stop(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().expect("Failed to kill Node Engine");
        }
    }

    pub async fn get_hash_string(&self, password: &str, salt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(&format!("http://localhost:3000/hashpwdstr?password={}&salt={}&iterations=100",password, salt))
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }

    pub async fn get_random_bytes(&self, bLen: u64) -> Result<String, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get(&format!("http://localhost:3000/getRandomBytes?len={}", bLen))
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }
}
