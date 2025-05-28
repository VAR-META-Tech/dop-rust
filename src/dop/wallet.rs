use crate::dop::DopClient;
use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::collections::HashMap;

impl DopClient {
    pub async fn generate_mnemonic(&self, words: Option<u8>) -> Result<String> {
        let url = match words {
            Some(24) => format!("{}/mnemonic?words=24", self.base_url()),
            _ => format!("{}/mnemonic", self.base_url()),
        };

        let res = self.client.get(&url).send().await?.json::<Value>().await?;
        res.get("mnemonic")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Mnemonic field missing or invalid in response"))
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

    pub async fn get_wallet_mnemonic(
        &self,
        wallet_id: &str,
        encryption_key: &str,
    ) -> Result<String> {
        let res = self
            .client
            .get(&format!(
                "{}/wallet/{}/mnemonic?encryptionKey={}",
                self.base_url(),
                wallet_id,
                encryption_key
            ))
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("mnemonic")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Missing mnemonic in response"))
    }

    pub async fn get_shareable_viewing_key(&self, wallet_id: &str) -> Result<String> {
        let res = self
            .client
            .get(&format!(
                "{}/wallet/{}/shareable-viewing-key",
                self.base_url(),
                wallet_id
            ))
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("shareableViewingKey")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Missing shareableViewingKey in response"))
    }

    pub async fn create_view_only_wallet(
        &self,
        encryption_key: &str,
        shareable_viewing_key: &str,
        creation_block_numbers: Option<HashMap<&str, u64>>,
    ) -> Result<Value> {
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
        let payload = json!({
            "walletId": wallet_id,
            "message": message
        });

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
            .ok_or_else(|| anyhow!("Missing signature in response"))
    }

    pub async fn load_wallet_by_id(
        &self,
        encryption_key: &str,
        wallet_id: &str,
        is_view_only: bool,
    ) -> Result<Value> {
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

    pub async fn unload_wallet_by_id(&self, wallet_id: &str) -> Result<()> {
        self.client
            .get(&format!("{}/wallet/{}/unload", self.base_url(), wallet_id))
            .send()
            .await?
            .error_for_status()?; // Ensure 2xx response
        Ok(())
    }

    pub async fn delete_wallet_by_id(&self, wallet_id: &str) -> Result<()> {
        self.client
            .delete(&format!("{}/wallet/{}/delete", self.base_url(), wallet_id))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn get_dop_wallet_address_data(&self, address: &str) -> Result<Value> {
        let res = self
            .client
            .get(&format!(
                "{}/wallet/address-data?address={}",
                self.base_url(),
                address
            ))
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(res)
    }

    pub async fn get_private_viewing_key(&self, wallet_id: &str) -> Result<String> {
        let res = self
            .client
            .get(&format!(
                "{}/wallet/{}/private-viewing-key",
                self.base_url(),
                wallet_id
            ))
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("privateViewingKey")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Missing privateViewingKey in response"))
    }

    pub async fn get_dop_address(&self, wallet_id: &str) -> Result<String> {
        let res = self
            .client
            .get(&format!(
                "{}/wallet/{}/dop-address",
                self.base_url(),
                wallet_id
            ))
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("dopAddress")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Missing dopAddress in response"))
    }

    pub async fn validate_dop_address(&self, address: &str) -> Result<bool> {
        let res = self
            .client
            .get(&format!(
                "{}/validate/dop-address?address={}",
                self.base_url(),
                address
            ))
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("valid")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| anyhow!("Missing validation result for DOP address"))
    }

    pub async fn validate_eth_address(&self, address: &str) -> Result<bool> {
        let res = self
            .client
            .get(&format!(
                "{}/validate/eth-address?address={}",
                self.base_url(),
                address
            ))
            .send()
            .await?
            .json::<Value>()
            .await?;

        res.get("valid")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| anyhow!("Missing validation result for ETH address"))
    }

    pub async fn scan_wallet(&self, wallet_id: &str, chain: Value) -> Result<Value> {
        let res = self
            .client
            .get(&format!("{}/wallet/{}/scan", self.base_url(), wallet_id))
            .query(&[("chain", chain.to_string())])
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(res)
    }

    pub async fn scan_wallet_multiple(
        &self,
        wallet_id: &str,
        chain: Value,
        count: u32,
    ) -> Result<()> {
        self.client
            .get(&format!(
                "{}/wallet/{}/scan-multiple",
                self.base_url(),
                wallet_id
            ))
            .query(&[("chain", chain.to_string()), ("count", count.to_string())])
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn assert_valid_eth_address(&self, address: &str) -> Result<()> {
        let res = self
            .client
            .get(&format!(
                "{}/assert/eth-address?address={}",
                self.base_url(),
                address
            ))
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("Invalid ETH address format"))
        }
    }

    pub async fn assert_valid_dop_address(&self, address: &str) -> Result<()> {
        let res = self
            .client
            .get(&format!(
                "{}/assert/dop-address?address={}",
                self.base_url(),
                address
            ))
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("Invalid DOP address format"))
        }
    }
}
