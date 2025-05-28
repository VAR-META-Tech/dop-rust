import express from "express";
import {
  gasEstimateForEncryptBaseToken,
  populateEncryptBaseToken,
  getEncryptPrivateKeySignatureMessage,
  generateEncryptTransaction,
  populateEncrypt,
  gasEstimateForEncrypt,
} from "dop-wallet-v3";
import { parseChain } from "../utils/json.js"; // your helper

export const txEncyptRouter = express.Router();

// Helper to safely JSON BigInt responses
function safeJsonResponse(res: express.Response, data: any) {
  res.setHeader("Content-Type", "application/json");
  res.send(
    JSON.stringify(data, (key, value) =>
      typeof value === "bigint" ? value.toString() : value
    )
  );
}

// POST /wallet/gas-estimate-for-encrypt-base-token
txEncyptRouter.post(
  "/gas-estimate-for-encrypt-base-token",
  async (req, res) => {
    try {
      const {
        txidVersion,
        networkName,
        dopAddress,
        encryptPrivateKey,
        wrappedERC20Amount,
        fromWalletAddress,
      } = req.body;
      if (
        !txidVersion ||
        !networkName ||
        !dopAddress ||
        !encryptPrivateKey ||
        !wrappedERC20Amount ||
        !fromWalletAddress
      ) {
        res.status(400).send("Missing required fields");
        return;
      }

      const estimate = await gasEstimateForEncryptBaseToken(
        txidVersion,
        networkName,
        dopAddress,
        encryptPrivateKey,
        wrappedERC20Amount,
        fromWalletAddress
      );
      safeJsonResponse(res, estimate);
    } catch (err) {
      console.error("❌ Failed to estimate gas:", err);
      res.status(500).json({
        error: "Failed to estimate gas for encrypt base token",
        details: (err as Error)?.message ?? String(err),
      });
    }
  }
);

// POST /wallet/populate-encrypt-base-token
txEncyptRouter.post("/populate-encrypt-base-token", async (req, res) => {
  try {
    const {
      txidVersion,
      networkName,
      dopAddress,
      encryptPrivateKey,
      wrappedERC20Amount,
      fromWalletAddress,
      gasDetails,
    } = req.body;
    if (
      !txidVersion ||
      !networkName ||
      !dopAddress ||
      !encryptPrivateKey ||
      !wrappedERC20Amount ||
      !fromWalletAddress
    ) {
      res.status(400).send("Missing required fields");
      return;
    }

    const tx = await populateEncryptBaseToken(
      txidVersion,
      networkName,
      dopAddress,
      encryptPrivateKey,
      wrappedERC20Amount,
      fromWalletAddress,
      gasDetails
    );
    safeJsonResponse(res, tx);
  } catch (err) {
    console.error("❌ Failed to populate encrypt base token:", err);
    res.status(500).json({
      error: "Failed to populate encrypt base token",
      details: (err as Error)?.message ?? String(err),
    });
  }
});

// POST /wallet/get-encrypt-private-key-signature-message
txEncyptRouter.post(
  "/get-encrypt-private-key-signature-message",
  async (_req, res) => {
    try {
      const message = getEncryptPrivateKeySignatureMessage();
      res.json({ message });
    } catch (err) {
      console.error("❌ Failed to get signature message:", err);
      res.status(500).json({
        error: "Failed to get encrypt signature message",
        details: (err as Error)?.message ?? String(err),
      });
    }
  }
);

// POST /wallet/generate-encrypt-transaction
txEncyptRouter.post("/generate-encrypt-transaction", async (req, res) => {
  try {
    const {
      txidVersion,
      networkName,
      encryptPrivateKey,
      erc20AmountRecipients,
      nftAmountRecipients,
    } = req.body;
    if (!txidVersion || !networkName || !encryptPrivateKey) {
      res.status(400).send("Missing required fields");
      return;
    }

    const tx = await generateEncryptTransaction(
      txidVersion,
      networkName,
      encryptPrivateKey,
      erc20AmountRecipients,
      nftAmountRecipients
    );
    safeJsonResponse(res, tx);
  } catch (err) {
    console.error("❌ Failed to generate encrypt transaction:", err);
    res.status(500).json({
      error: "Failed to generate encrypt transaction",
      details: (err as Error)?.message ?? String(err),
    });
  }
});

// POST /wallet/populate-encrypt
txEncyptRouter.post("/populate-encrypt", async (req, res) => {
  try {
    const {
      txidVersion,
      networkName,
      encryptPrivateKey,
      erc20AmountRecipients,
      nftAmountRecipients,
      gasDetails,
    } = req.body;
    if (!txidVersion || !networkName || !encryptPrivateKey) {
      res.status(400).send("Missing required fields");
      return;
    }

    const tx = await populateEncrypt(
      txidVersion,
      networkName,
      encryptPrivateKey,
      erc20AmountRecipients,
      nftAmountRecipients,
      gasDetails
    );
    safeJsonResponse(res, tx);
  } catch (err) {
    console.error("❌ Failed to populate encrypt transaction:", err);
    res.status(500).json({
      error: "Failed to populate encrypt",
      details: (err as Error)?.message ?? String(err),
    });
  }
});

// POST /wallet/gas-estimate-for-encrypt
txEncyptRouter.post("/gas-estimate-for-encrypt", async (req, res) => {
  try {
    const {
      txidVersion,
      networkName,
      encryptPrivateKey,
      erc20AmountRecipients,
      nftAmountRecipients,
      fromWalletAddress,
    } = req.body;
    if (
      !txidVersion ||
      !networkName ||
      !encryptPrivateKey ||
      !fromWalletAddress
    ) {
      res.status(400).send("Missing required fields");
      return;
    }

    const estimate = await gasEstimateForEncrypt(
      txidVersion,
      networkName,
      encryptPrivateKey,
      erc20AmountRecipients,
      nftAmountRecipients,
      fromWalletAddress
    );
    safeJsonResponse(res, estimate);
  } catch (err) {
    console.error("❌ Failed to estimate gas for encrypt:", err);
    res.status(500).json({
      error: "Failed to estimate gas for encrypt",
      details: (err as Error)?.message ?? String(err),
    });
  }
});
