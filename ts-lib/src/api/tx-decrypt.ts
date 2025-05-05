import express from "express";
import {
  populateProvedDecrypt,
  populateProvedDecryptBaseToken,
  gasEstimateForUnprovenDecrypt,
  gasEstimateForUnprovenDecryptBaseToken,
  getERC20AndNFTAmountRecipientsForDecryptToOrigin,
  populateProvedDecryptToOrigin,
  gasEstimateForUnprovenDecryptToOrigin,
  generateDecryptProof,
  generateDecryptToOriginProof,
  generateDecryptBaseTokenProof,
} from "dop-wallet-v3";

export const txDecryptRouter = express.Router();

function safeJsonResponse(res: express.Response, data: any) {
  res.setHeader("Content-Type", "application/json");
  res.send(
    JSON.stringify(data, (_, v) => (typeof v === "bigint" ? v.toString() : v))
  );
}

// POST /wallet/generate-decrypt-proof
txDecryptRouter.post("/generate-decrypt-proof", async (req, res) => {
  try {
    await generateDecryptProof(
      req.body.txidVersion,
      req.body.networkName,
      req.body.dopWalletID,
      req.body.encryptionKey,
      req.body.erc20AmountRecipients,
      req.body.nftAmountRecipients,
      req.body.broadcasterFeeERC20AmountRecipient,
      req.body.sendWithPublicWallet,
      req.body.overallBatchMinGasPrice
        ? BigInt(req.body.overallBatchMinGasPrice)
        : null,
      () => {}, // Empty callback
      BigInt(req.body.value)
    );
    res.json({ success: true });
  } catch (err) {
    res.status(500).json({
      error: "generateDecryptProof failed",
      details: String(err),
    });
  }
});

// POST /wallet/generate-decrypt-to-origin-proof
txDecryptRouter.post("/generate-decrypt-to-origin-proof", async (req, res) => {
  try {
    await generateDecryptToOriginProof(
      req.body.originalEncryptTxid,
      req.body.txidVersion,
      req.body.networkName,
      req.body.dopWalletID,
      req.body.encryptionKey,
      req.body.erc20AmountRecipients,
      req.body.nftAmountRecipients,
      () => {}, // Empty callback
      BigInt(req.body.value)
    );
    res.json({ success: true });
  } catch (err) {
    res.status(500).json({
      error: "generateDecryptToOriginProof failed",
      details: String(err),
    });
  }
});

// POST /wallet/generate-decrypt-base-token-proof
txDecryptRouter.post("/generate-decrypt-base-token-proof", async (req, res) => {
  try {
    await generateDecryptBaseTokenProof(
      req.body.txidVersion,
      req.body.networkName,
      req.body.publicWalletAddress,
      req.body.dopWalletID,
      req.body.encryptionKey,
      req.body.wrappedERC20Amount,
      req.body.broadcasterFeeERC20AmountRecipient,
      req.body.sendWithPublicWallet,
      req.body.overallBatchMinGasPrice
        ? BigInt(req.body.overallBatchMinGasPrice)
        : null,
      () => {} // Empty callback
    );
    res.json({ success: true });
  } catch (err) {
    res.status(500).json({
      error: "generateDecryptBaseTokenProof failed",
      details: String(err),
    });
  }
});

// POST /wallet/populate-proved-decrypt
txDecryptRouter.post("/populate-proved-decrypt", async (req, res) => {
  try {
    const tx = await populateProvedDecrypt(
      req.body.txidVersion,
      req.body.networkName,
      req.body.dopWalletID,
      req.body.erc20AmountRecipients,
      req.body.nftAmountRecipients,
      req.body.broadcasterFeeERC20AmountRecipient,
      req.body.sendWithPublicWallet,
      req.body.overallBatchMinGasPrice
        ? BigInt(req.body.overallBatchMinGasPrice)
        : null,
      req.body.gasDetails
    );
    safeJsonResponse(res, tx);
  } catch (err) {
    res
      .status(500)
      .json({ error: "populateProvedDecrypt failed", details: String(err) });
  }
});

// POST /wallet/populate-proved-decrypt-base-token
txDecryptRouter.post(
  "/populate-proved-decrypt-base-token",
  async (req, res) => {
    try {
      const tx = await populateProvedDecryptBaseToken(
        req.body.txidVersion,
        req.body.networkName,
        req.body.publicWalletAddress,
        req.body.dopWalletID,
        req.body.wrappedERC20Amount,
        req.body.broadcasterFeeERC20AmountRecipient,
        req.body.sendWithPublicWallet,
        req.body.overallBatchMinGasPrice
          ? BigInt(req.body.overallBatchMinGasPrice)
          : null,
        req.body.gasDetails
      );
      safeJsonResponse(res, tx);
    } catch (err) {
      res.status(500).json({
        error: "populateProvedDecryptBaseToken failed",
        details: String(err),
      });
    }
  }
);

// POST /wallet/gas-estimate-for-unproven-decrypt
txDecryptRouter.post("/gas-estimate-for-unproven-decrypt", async (req, res) => {
  try {
    const estimate = await gasEstimateForUnprovenDecrypt(
      req.body.txidVersion,
      req.body.networkName,
      req.body.dopWalletID,
      req.body.encryptionKey,
      req.body.erc20AmountRecipients,
      req.body.nftAmountRecipients,
      req.body.originalGasDetails,
      req.body.feeTokenDetails,
      req.body.sendWithPublicWallet,
      BigInt(req.body.value)
    );
    safeJsonResponse(res, estimate);
  } catch (err) {
    res.status(500).json({
      error: "gasEstimateForUnprovenDecrypt failed",
      details: String(err),
    });
  }
});

// POST /wallet/gas-estimate-for-unproven-decrypt-base-token
txDecryptRouter.post(
  "/gas-estimate-for-unproven-decrypt-base-token",
  async (req, res) => {
    try {
      const estimate = await gasEstimateForUnprovenDecryptBaseToken(
        req.body.txidVersion,
        req.body.networkName,
        req.body.publicWalletAddress,
        req.body.dopWalletID,
        req.body.encryptionKey,
        req.body.wrappedERC20Amount,
        req.body.originalGasDetails,
        req.body.feeTokenDetails,
        req.body.sendWithPublicWallet
      );
      safeJsonResponse(res, estimate);
    } catch (err) {
      res.status(500).json({
        error: "gasEstimateForUnprovenDecryptBaseToken failed",
        details: String(err),
      });
    }
  }
);

// POST /wallet/get-recipients-for-decrypt-to-origin
txDecryptRouter.post(
  "/get-recipients-for-decrypt-to-origin",
  async (req, res) => {
    try {
      const recipients = await getERC20AndNFTAmountRecipientsForDecryptToOrigin(
        req.body.txidVersion,
        req.body.networkName,
        req.body.dopWalletID,
        req.body.originalEncryptTxid
      );
      res.json(recipients);
    } catch (err) {
      res.status(500).json({
        error: "getRecipientsForDecryptToOrigin failed",
        details: String(err),
      });
    }
  }
);

// POST /wallet/populate-proved-decrypt-to-origin
txDecryptRouter.post("/populate-proved-decrypt-to-origin", async (req, res) => {
  try {
    const tx = await populateProvedDecryptToOrigin(
      req.body.txidVersion,
      req.body.networkName,
      req.body.dopWalletID,
      req.body.erc20AmountRecipients,
      req.body.nftAmountRecipients,
      req.body.gasDetails
    );
    safeJsonResponse(res, tx);
  } catch (err) {
    res.status(500).json({
      error: "populateProvedDecryptToOrigin failed",
      details: String(err),
    });
  }
});

// POST /wallet/gas-estimate-for-unproven-decrypt-to-origin
txDecryptRouter.post(
  "/gas-estimate-for-unproven-decrypt-to-origin",
  async (req, res) => {
    try {
      const estimate = await gasEstimateForUnprovenDecryptToOrigin(
        req.body.originalEncryptTxid,
        req.body.txidVersion,
        req.body.networkName,
        req.body.dopWalletID,
        req.body.encryptionKey,
        req.body.erc20AmountRecipients,
        BigInt(req.body.value),
        req.body.nftAmountRecipients
      );
      safeJsonResponse(res, estimate);
    } catch (err) {
      res.status(500).json({
        error: "gasEstimateForUnprovenDecryptToOrigin failed",
        details: String(err),
      });
    }
  }
);
