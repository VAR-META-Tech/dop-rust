use crate::dop::DopClient;
use anyhow::{Context, Result};
use serde_json::{json, Value};

impl DopClient {
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
