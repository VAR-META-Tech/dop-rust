import express from "express";
import {
  populateProvedTransfer,
  gasEstimateForUnprovenTransfer,
  generateTransferProof,
} from "dop-wallet-v3";

export const txTransferRouter = express.Router();

function safeJsonResponse(res: express.Response, data: any) {
  res.setHeader("Content-Type", "application/json");
  res.send(
    JSON.stringify(data, (key, value) =>
      typeof value === "bigint" ? value.toString() : value
    )
  );
}

// POST /wallet/populate-proved-transfer
txTransferRouter.post("/populate-proved-transfer", async (req, res) => {
  try {
    const {
      txidVersion,
      networkName,
      dopWalletID,
      showSenderAddressToRecipient,
      memoText,
      erc20AmountRecipients,
      nftAmountRecipients,
      broadcasterFeeERC20AmountRecipient,
      sendWithPublicWallet,
      overallBatchMinGasPrice,
      gasDetails,
    } = req.body;

    if (!txidVersion || !networkName || !dopWalletID || !gasDetails) {
      res.status(400).send("Missing required fields");
      return;
    }

    const tx = await populateProvedTransfer(
      txidVersion,
      networkName,
      dopWalletID,
      showSenderAddressToRecipient ?? false,
      memoText,
      erc20AmountRecipients ?? [],
      nftAmountRecipients ?? [],
      broadcasterFeeERC20AmountRecipient,
      sendWithPublicWallet ?? false,
      overallBatchMinGasPrice,
      gasDetails
    );

    safeJsonResponse(res, tx);
  } catch (err) {
    console.error("❌ Failed to populate proved transfer:", err);
    res.status(500).json({
      error: "Failed to populate proved transfer",
      details: (err as Error)?.message ?? String(err),
    });
  }
});

// POST /wallet/gas-estimate-for-unproven-transfer
txTransferRouter.post(
  "/gas-estimate-for-unproven-transfer",
  async (req, res) => {
    try {
      const {
        txidVersion,
        networkName,
        dopWalletID,
        encryptionKey,
        memoText,
        erc20AmountRecipients,
        nftAmountRecipients,
        originalGasDetails,
        feeTokenDetails,
        sendWithPublicWallet,
      } = req.body;

      if (
        !txidVersion ||
        !networkName ||
        !dopWalletID ||
        !encryptionKey ||
        !originalGasDetails
      ) {
        res.status(400).send("Missing required fields");
        return;
      }

      const estimate = await gasEstimateForUnprovenTransfer(
        txidVersion,
        networkName,
        dopWalletID,
        encryptionKey,
        memoText,
        erc20AmountRecipients ?? [],
        nftAmountRecipients ?? [],
        originalGasDetails,
        feeTokenDetails,
        sendWithPublicWallet ?? false
      );

      safeJsonResponse(res, estimate);
    } catch (err) {
      console.error("❌ Failed to estimate gas for unproven transfer:", err);
      res.status(500).json({
        error: "Failed to estimate gas for unproven transfer",
        details: (err as Error)?.message ?? String(err),
      });
    }
  }
);

txTransferRouter.post("/generate-transfer-proof", async (req, res) => {
  try {
    const {
      txidVersion,
      networkName,
      dopWalletID,
      encryptionKey,
      showSenderAddressToRecipient,
      memoText,
      erc20AmountRecipients,
      nftAmountRecipients,
      broadcasterFeeERC20AmountRecipient,
      sendWithPublicWallet,
      overallBatchMinGasPrice,
    } = req.body;

    if (!txidVersion || !networkName || !dopWalletID || !encryptionKey) {
      res.status(400).send("Missing required fields");
      return;
    }

    await generateTransferProof(
      txidVersion,
      networkName,
      dopWalletID,
      encryptionKey,
      showSenderAddressToRecipient ?? false,
      memoText,
      erc20AmountRecipients ?? [],
      nftAmountRecipients ?? [],
      broadcasterFeeERC20AmountRecipient,
      sendWithPublicWallet ?? false,
      overallBatchMinGasPrice,
      (_progress) => {
        console.log("🔄 Generating transfer proof...");
      }
    );

    res.status(200).json({ message: "Transfer proof generation complete" });
  } catch (err) {
    console.error("❌ Failed to generate transfer proof:", err);
    res.status(500).json({
      error: "Failed to generate transfer proof",
      details: (err as Error)?.message ?? String(err),
    });
  }
});
