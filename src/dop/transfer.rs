use crate::dop::DopClient;
use anyhow::Result;
use serde_json::{json, Value};

impl DopClient {
    pub async fn gas_estimate_for_unproven_transfer(
        &self,
        txid_version: &str,
        network: &str,
        wallet_id: &str,
        encryption_key: &str,
        memo: &str,
        token_amount_recipients: Vec<Value>,
        nft_amount_recipients: Vec<Value>,
        tx_gas_details: Value,
        fee_token_details: Value,
        send_with_public_wallet: bool,
    ) -> Result<Value> {
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
