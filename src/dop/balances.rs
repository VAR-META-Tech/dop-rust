use crate::dop::DopClient;
use anyhow::Result;
use serde_json::{Value, json};

impl DopClient {
    pub async fn refresh_balances(
        &self,
        chain: Value,
        wallet_ids: Option<Vec<String>>,
    ) -> Result<()> {
        println!("Refreshing balances for chain: {}", chain);

        let payload = json!({
            "chain": chain,
            "walletIdFilter": wallet_ids,
        });

        self.client
            .post(&format!("{}/refresh-balances", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn rescan_full_utxo_merkletrees_and_wallets(
        &self,
        chain: Value,
        wallet_ids: Option<Vec<String>>,
    ) -> Result<()> {
        println!(
            "Rescanning UTXO merkletrees and wallets for chain: {}",
            chain
        );

        let payload = json!({
            "chain": chain,
            "walletIdFilter": wallet_ids,
        });

        self.client
            .post(&format!("{}/rescan-full-utxo-merkletrees", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn reset_full_txid_merkletrees_v2(&self, chain: Value) -> Result<()> {
        println!("Resetting full TXID merkletrees V2 for chain: {}", chain);

        let payload = json!({
            "chain": chain,
        });

        self.client
            .post(&format!("{}/reset-full-txid-merkletrees", self.base_url()))
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
