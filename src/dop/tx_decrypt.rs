use crate::dop::{
    DopClient, DopERC20Amount, DopERC20AmountRecipient, DopNFTAmountRecipient,
    DopPopulateTransactionResponse, DopTransactionGasEstimateResponse, FeeTokenDetails,
    TransactionGasDetails,
};
use anyhow::{Result, anyhow};
use serde_json::json;

impl DopClient {
    pub async fn generate_decrypt_proof(
        &self,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        encryption_key: String,
        erc20_amount_recipients: Vec<DopERC20AmountRecipient>,
        nft_amount_recipients: Vec<DopNFTAmountRecipient>,
        broadcaster_fee_recipient: Option<DopERC20AmountRecipient>,
        send_with_public_wallet: bool,
        overall_batch_min_gas_price: Option<String>,
        value: String,
    ) -> Result<()> {
        let payload = json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "encryptionKey": encryption_key,
            "erc20AmountRecipients": erc20_amount_recipients,
            "nftAmountRecipients": nft_amount_recipients,
            "broadcasterFeeERC20AmountRecipient": broadcaster_fee_recipient,
            "sendWithPublicWallet": send_with_public_wallet,
            "overallBatchMinGasPrice": overall_batch_min_gas_price,
            "value": value
        });

        self.send_request::<serde_json::Value>("/generate-decrypt-proof", payload)
            .await?;
        Ok(())
    }

    pub async fn generate_decrypt_to_origin_proof(
        &self,
        original_encrypt_txid: String,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        encryption_key: String,
        erc20_amount_recipients: Vec<DopERC20AmountRecipient>,
        nft_amount_recipients: Vec<DopNFTAmountRecipient>,
        value: String,
    ) -> Result<()> {
        let payload = json!({
            "originalEncryptTxid": original_encrypt_txid,
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "encryptionKey": encryption_key,
            "erc20AmountRecipients": erc20_amount_recipients,
            "nftAmountRecipients": nft_amount_recipients,
            "value": value
        });

        self.send_request::<serde_json::Value>("/generate-decrypt-to-origin-proof", payload)
            .await?;
        Ok(())
    }

    pub async fn generate_decrypt_base_token_proof(
        &self,
        txid_version: String,
        network_name: String,
        public_wallet_address: String,
        dop_wallet_id: String,
        encryption_key: String,
        wrapped_erc20_amount: DopERC20Amount,
        broadcaster_fee_recipient: Option<DopERC20AmountRecipient>,
        send_with_public_wallet: bool,
        overall_batch_min_gas_price: Option<String>,
    ) -> Result<()> {
        let payload = json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "publicWalletAddress": public_wallet_address,
            "dopWalletID": dop_wallet_id,
            "encryptionKey": encryption_key,
            "wrappedERC20Amount": wrapped_erc20_amount,
            "broadcasterFeeERC20AmountRecipient": broadcaster_fee_recipient,
            "sendWithPublicWallet": send_with_public_wallet,
            "overallBatchMinGasPrice": overall_batch_min_gas_price
        });

        self.send_request::<serde_json::Value>("/generate-decrypt-base-token-proof", payload)
            .await?;
        Ok(())
    }

    pub async fn populate_proved_decrypt(
        &self,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        nft_recipients: Vec<DopNFTAmountRecipient>,
        broadcaster_fee_recipient: Option<DopERC20AmountRecipient>,
        send_with_public_wallet: bool,
        overall_batch_min_gas_price: Option<String>,
        gas_details: TransactionGasDetails,
    ) -> Result<DopPopulateTransactionResponse> {
        let mut payload = json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "erc20AmountRecipients": erc20_recipients,
            "nftAmountRecipients": nft_recipients,
            "sendWithPublicWallet": send_with_public_wallet,
            "gasDetails": gas_details
        });

        if let Some(recipient) = broadcaster_fee_recipient {
            payload.as_object_mut().unwrap().insert(
                "broadcasterFeeERC20AmountRecipient".to_string(),
                serde_json::to_value(recipient)?,
            );
        }

        if let Some(min_gas) = overall_batch_min_gas_price {
            payload
                .as_object_mut()
                .unwrap()
                .insert("overallBatchMinGasPrice".to_string(), json!(min_gas));
        }

        self.send_request("/populate-proved-decrypt", payload).await
    }

    pub async fn populate_proved_decrypt_base_token(
        &self,
        txid_version: String,
        network_name: String,
        public_wallet_address: String,
        dop_wallet_id: String,
        wrapped_erc20_amount: DopERC20Amount,
        broadcaster_fee_recipient: Option<DopERC20AmountRecipient>,
        send_with_public_wallet: bool,
        overall_batch_min_gas_price: Option<String>,
        gas_details: TransactionGasDetails,
    ) -> Result<DopPopulateTransactionResponse> {
        let mut payload = json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "publicWalletAddress": public_wallet_address,
            "dopWalletID": dop_wallet_id,
            "wrappedERC20Amount": wrapped_erc20_amount,
            "sendWithPublicWallet": send_with_public_wallet,
            "gasDetails": gas_details,
        });

        if let Some(recipient) = broadcaster_fee_recipient {
            payload.as_object_mut().unwrap().insert(
                "broadcasterFeeERC20AmountRecipient".to_string(),
                serde_json::to_value(recipient)?,
            );
        }

        if let Some(min_gas) = overall_batch_min_gas_price {
            payload
                .as_object_mut()
                .unwrap()
                .insert("overallBatchMinGasPrice".to_string(), json!(min_gas));
        }

        self.send_request("/populate-proved-decrypt-base-token", payload)
            .await
    }

    pub async fn gas_estimate_for_unproven_decrypt(
        &self,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        encryption_key: String,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        nft_recipients: Vec<DopNFTAmountRecipient>,
        original_gas_details: TransactionGasDetails,
        fee_token_details: Option<FeeTokenDetails>,
        send_with_public_wallet: bool,
        value: String,
    ) -> Result<DopTransactionGasEstimateResponse> {
        let mut payload = json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "encryptionKey": encryption_key,
            "erc20AmountRecipients": erc20_recipients,
            "nftAmountRecipients": nft_recipients,
            "originalGasDetails": original_gas_details,
            "sendWithPublicWallet": send_with_public_wallet,
            "value": value
        });

        if let Some(fee_token) = fee_token_details {
            payload.as_object_mut().unwrap().insert(
                "feeTokenDetails".to_string(),
                serde_json::to_value(fee_token)?,
            );
        }

        self.send_request("/gas-estimate-for-unproven-decrypt", payload)
            .await
    }

    pub async fn gas_estimate_for_unproven_decrypt_base_token(
        &self,
        txid_version: String,
        network_name: String,
        public_wallet_address: String,
        dop_wallet_id: String,
        encryption_key: String,
        wrapped_erc20_amount: DopERC20Amount,
        original_gas_details: TransactionGasDetails,
        fee_token_details: Option<FeeTokenDetails>,
        send_with_public_wallet: bool,
    ) -> Result<DopTransactionGasEstimateResponse> {
        let mut payload = json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "publicWalletAddress": public_wallet_address,
            "dopWalletID": dop_wallet_id,
            "encryptionKey": encryption_key,
            "wrappedERC20Amount": wrapped_erc20_amount,
            "originalGasDetails": original_gas_details,
            "sendWithPublicWallet": send_with_public_wallet
        });

        if let Some(fee_token) = fee_token_details {
            payload.as_object_mut().unwrap().insert(
                "feeTokenDetails".to_string(),
                serde_json::to_value(fee_token)?,
            );
        }

        self.send_request("/gas-estimate-for-unproven-decrypt-base-token", payload)
            .await
    }

    pub async fn get_erc20_and_nft_amount_recipients_for_decrypt_to_origin(
        &self,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        original_encrypt_txid: String,
    ) -> Result<(Vec<DopERC20AmountRecipient>, Vec<DopNFTAmountRecipient>)> {
        let payload = json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "originalEncryptTxid": original_encrypt_txid
        });

        let resp = self
            .client
            .post(&format!(
                "{}/get-recipients-for-decrypt-to-origin",
                self.base_url()
            ))
            .json(&payload)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("API call failed: {} (status {})", text, status));
        }

        let json: serde_json::Value = resp.json().await?;
        let erc20 = serde_json::from_value(json["erc20AmountRecipients"].clone())?;
        let nft = serde_json::from_value(json["nftAmountRecipients"].clone())?;
        Ok((erc20, nft))
    }

    pub async fn populate_proved_decrypt_to_origin(
        &self,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        nft_recipients: Vec<DopNFTAmountRecipient>,
        gas_details: TransactionGasDetails,
    ) -> Result<DopPopulateTransactionResponse> {
        let payload = json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "erc20AmountRecipients": erc20_recipients,
            "nftAmountRecipients": nft_recipients,
            "gasDetails": gas_details
        });

        self.send_request("/populate-proved-decrypt-to-origin", payload)
            .await
    }

    pub async fn gas_estimate_for_unproven_decrypt_to_origin(
        &self,
        original_encrypt_txid: String,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        encryption_key: String,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        value: String,
        nft_recipients: Vec<DopNFTAmountRecipient>,
    ) -> Result<DopTransactionGasEstimateResponse> {
        let payload = json!({
            "originalEncryptTxid": original_encrypt_txid,
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "encryptionKey": encryption_key,
            "erc20AmountRecipients": erc20_recipients,
            "value": value,
            "nftAmountRecipients": nft_recipients
        });

        self.send_request("/gas-estimate-for-unproven-decrypt-to-origin", payload)
            .await
    }

    async fn send_request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        endpoint: &str,
        payload: serde_json::Value,
    ) -> Result<T> {
        let resp = self
            .client
            .post(&format!("{}{}", self.base_url(), endpoint))
            .json(&payload)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("API call failed: {} (status {})", text, status));
        }

        Ok(resp.json::<T>().await?)
    }
}
