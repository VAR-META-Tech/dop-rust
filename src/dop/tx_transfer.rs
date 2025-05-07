use super::{
    DopClient, DopERC20AmountRecipient, DopNFTAmountRecipient,
    DopPopulateTransactionResponse, DopTransactionGasEstimateResponse, FeeTokenDetails,
    TransactionGasDetails,
};
use serde_json::json;

impl DopClient {
    pub async fn populate_proved_transfer(
        &self,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        show_sender_address_to_recipient: bool,
        memo_text: Option<String>,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        nft_recipients: Vec<DopNFTAmountRecipient>,
        broadcaster_fee_recipient: Option<DopERC20AmountRecipient>,
        send_with_public_wallet: bool,
        overall_batch_min_gas_price: Option<String>,
        gas_details: TransactionGasDetails,
    ) -> anyhow::Result<DopPopulateTransactionResponse> {
        let gas_details_json = serde_json::to_value(gas_details)?;

        let payload = serde_json::json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "showSenderAddressToRecipient": show_sender_address_to_recipient,
            "memoText": memo_text,
            "erc20AmountRecipients": erc20_recipients,
            "nftAmountRecipients": nft_recipients,
            "broadcasterFeeERC20AmountRecipient": broadcaster_fee_recipient,
            "sendWithPublicWallet": send_with_public_wallet,
            "overallBatchMinGasPrice": overall_batch_min_gas_price,
            "gasDetails": gas_details_json,
        });

        let resp = self
            .client
            .post(&format!("{}/populate-proved-transfer", self.base_url()))
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

    pub async fn gas_estimate_for_unproven_transfer(
        &self,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        encryption_key: String,
        memo_text: Option<String>,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        nft_recipients: Vec<DopNFTAmountRecipient>,
        original_gas_details: TransactionGasDetails,
        fee_token_details: Option<FeeTokenDetails>,
        send_with_public_wallet: bool,
    ) -> anyhow::Result<DopTransactionGasEstimateResponse> {
        let gas_details_json = serde_json::to_value(original_gas_details)?;

        let payload = serde_json::json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "encryptionKey": encryption_key,
            "memoText": memo_text,
            "erc20AmountRecipients": erc20_recipients,
            "nftAmountRecipients": nft_recipients,
            "originalGasDetails": gas_details_json,
            "feeTokenDetails": fee_token_details,
            "sendWithPublicWallet": send_with_public_wallet,
        });

        let resp = self
            .client
            .post(&format!(
                "{}/gas-estimate-for-unproven-transfer",
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

    pub async fn generate_transfer_proof(
        &self,
        txid_version: String,
        network_name: String,
        dop_wallet_id: String,
        encryption_key: String,
        show_sender_address_to_recipient: bool,
        memo_text: Option<String>,
        erc20_recipients: Vec<DopERC20AmountRecipient>,
        nft_recipients: Vec<DopNFTAmountRecipient>,
        broadcaster_fee_recipient: Option<DopERC20AmountRecipient>,
        send_with_public_wallet: bool,
        overall_batch_min_gas_price: Option<String>,
    ) -> Result<(), anyhow::Error> {
        let payload = json!({
            "txidVersion": txid_version,
            "networkName": network_name,
            "dopWalletID": dop_wallet_id,
            "encryptionKey": encryption_key,
            "showSenderAddressToRecipient": show_sender_address_to_recipient,
            "memoText": memo_text,
            "erc20AmountRecipients": erc20_recipients,
            "nftAmountRecipients": nft_recipients,
            "broadcasterFeeERC20AmountRecipient": broadcaster_fee_recipient,
            "sendWithPublicWallet": send_with_public_wallet,
            "overallBatchMinGasPrice": overall_batch_min_gas_price,
        });

        let resp = self
            .client
            .post(&format!("{}/generate-transfer-proof", self.base_url()))
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

        Ok(())
    }
}
