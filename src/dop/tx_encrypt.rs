use super::DopClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DopERC20AmountRecipient {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    pub amount: String,
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DopNFTAmountRecipient {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitmentSummary {
    pub commitment_ciphertext: serde_json::Value,
    pub commitment_hash: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContractTransaction {
    pub to: String,
    pub data: String,
    pub from: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DopPopulateTransactionResponse {
    pub transaction: ContractTransaction,
    pub nullifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "evmGasType")]
pub enum TransactionGasDetails {
    #[serde(rename = "0")]
    Type0 {
        #[serde(rename = "gasEstimate")]
        gas_estimate: String,
        #[serde(rename = "gasPrice")]
        gas_price: String,
    },
    #[serde(rename = "1")]
    Type1 {
        #[serde(rename = "gasEstimate")]
        gas_estimate: String,
        #[serde(rename = "gasPrice")]
        gas_price: String,
    },
    #[serde(rename = "2")]
    Type2 {
        #[serde(rename = "gasEstimate")]
        gas_estimate: String,
        #[serde(rename = "maxFeePerGas")]
        max_fee_per_gas: String,
        #[serde(rename = "maxPriorityFeePerGas")]
        max_priority_fee_per_gas: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DopERC20Amount {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeTokenDetails {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "maxAmount")]
    pub max_amount: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DopTransactionGasEstimateResponse {
    #[serde(rename = "gasEstimate")]
    pub gas_estimate: String,
    #[serde(rename = "broadcasterFeeCommitment")]
    pub broadcaster_fee_commitment: Option<CommitmentSummary>,
}

impl DopClient {
    // Encrypt Base Token - Gas Estimate
    pub async fn gas_estimate_for_encrypt_base_token(
        &self,
        txid_version: String,
        network_name: String,
        dop_address: String,
        encrypt_private_key: String,
        wrapped_erc20_amount: DopERC20Amount,
        from_wallet_address: String,
    ) -> anyhow::Result<DopTransactionGasEstimateResponse> {
        let payload = serde_json::json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopAddress": dop_address,
            "encryptPrivateKey": encrypt_private_key,
            "wrappedERC20Amount": wrapped_erc20_amount,
            "fromWalletAddress": from_wallet_address,
        });

        let resp = self
            .client
            .post(&format!(
                "{}/gas-estimate-for-encrypt-base-token",
                self.base_url()
            ))
            .json(&payload)
            .send()
            .await?;

        println!("Response status: {:?}", resp.status());
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "API call failed: {} (status {})",
                text,
                status
            ));
        }

        let result = resp.json::<DopTransactionGasEstimateResponse>().await?;
        Ok(result)
    }

    // Encrypt Base Token - Populate Transaction
    pub async fn populate_encrypt_base_token(
        &self,
        txid_version: String,
        network_name: String,
        dop_address: String,
        encrypt_private_key: String,
        wrapped_erc20_amount: DopERC20Amount,
        from_wallet_address: String,
        gas_details: Option<serde_json::Value>,
    ) -> anyhow::Result<DopPopulateTransactionResponse> {
        let mut payload = serde_json::json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopAddress": dop_address,
            "encryptPrivateKey": encrypt_private_key,
            "wrappedERC20Amount": wrapped_erc20_amount,
            "fromWalletAddress": from_wallet_address,
        });

        if let Some(details) = gas_details {
            payload
                .as_object_mut()
                .unwrap()
                .insert("gasDetails".to_string(), details);
        }

        let resp = self
            .client
            .post(&format!("{}/populate-encrypt-base-token", self.base_url()))
            .json(&payload)
            .send()
            .await?;

        println!("Response status: {:?}", resp.status());
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "API call failed: {} (status {})",
                text,
                status
            ));
        }

        let result = resp.json::<DopPopulateTransactionResponse>().await?;
        Ok(result)
    }

    // Encrypt - Gas Estimate
    pub async fn gas_estimate_for_encrypt(
        &self,
        txid_version: String,
        network_name: String,
        encrypt_private_key: String,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        nft_recipients: Vec<DopNFTAmountRecipient>,
        from_wallet_address: String,
    ) -> anyhow::Result<DopTransactionGasEstimateResponse> {
        let payload = serde_json::json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "encryptPrivateKey": encrypt_private_key,
            "erc20AmountRecipients": erc20_recipients,
            "nftAmountRecipients": nft_recipients,
            "fromWalletAddress": from_wallet_address,
        });

        let resp = self
            .client
            .post(&format!("{}/gas-estimate-for-encrypt", self.base_url()))
            .json(&payload)
            .send()
            .await?;

        println!("Response status: {:?}", resp.status());
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "API call failed: {} (status {})",
                text,
                status
            ));
        }

        let result = resp.json::<DopTransactionGasEstimateResponse>().await?;
        Ok(result)
    }

    // Encrypt - Populate Transaction
    pub async fn populate_encrypt(
        &self,
        txid_version: String,
        network_name: String,
        encrypt_private_key: String,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        nft_recipients: Vec<DopNFTAmountRecipient>,
        gas_details: Option<serde_json::Value>,
    ) -> anyhow::Result<DopPopulateTransactionResponse> {
        let mut payload = serde_json::json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "encryptPrivateKey": encrypt_private_key,
            "erc20AmountRecipients": erc20_recipients,
            "nftAmountRecipients": nft_recipients,
        });

        if let Some(details) = gas_details {
            payload
                .as_object_mut()
                .unwrap()
                .insert("gasDetails".to_string(), details);
        }

        let resp = self
            .client
            .post(&format!("{}/populate-encrypt", self.base_url()))
            .json(&payload)
            .send()
            .await?;

        println!("Response status: {:?}", resp.status());
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "API call failed: {} (status {})",
                text,
                status
            ));
        }

        let result = resp.json::<DopPopulateTransactionResponse>().await?;
        Ok(result)
    }

    // Encrypt - Get Signature Message
    pub async fn get_encrypt_private_key_signature_message(&self) -> anyhow::Result<String> {
        let resp = self
            .client
            .post(&format!(
                "{}/get-encrypt-private-key-signature-message",
                self.base_url()
            ))
            .send()
            .await?;

        println!("Response status: {:?}", resp.status());
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "API call failed: {} (status {})",
                text,
                status
            ));
        }

        let json = resp.json::<serde_json::Value>().await?;
        let message = json["message"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing message in response"))?
            .to_string();
        Ok(message)
    }

    pub async fn generate_encrypt_transaction(
        &self,
        txid_version: String,
        network_name: String,
        encrypt_private_key: String,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        nft_recipients: Vec<DopNFTAmountRecipient>,
    ) -> anyhow::Result<ContractTransaction> {
        let payload = serde_json::json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "encryptPrivateKey": encrypt_private_key,
            "erc20AmountRecipients": erc20_recipients,
            "nftAmountRecipients": nft_recipients,
        });

        let resp = self
            .client
            .post(&format!("{}/generate-encrypt-transaction", self.base_url()))
            .json(&payload)
            .send()
            .await?;

        println!("Response status: {:?}", resp.status());
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "API call failed: {} (status {})",
                text,
                status
            ));
        }

        let tx = resp.json::<ContractTransaction>().await?;
        Ok(tx)
    }
}
