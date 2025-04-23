use crate::dop::DopClient;
use anyhow::{Context, Result};
use serde_json::Value;
use tokio::time::{Duration, sleep};
use anyhow::anyhow;

impl DopClient {
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
        let payload = serde_json::json!({
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
        self.client
            .get(&format!("{}/close", self.base_url()))
            .send()
            .await?;
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
            Err(anyhow::anyhow!(
                "Invalid engine info response: wallets missing"
            ))
        } else {
            Ok(res)
        }
    }
}
