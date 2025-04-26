import express from "express";
import {
  initEngine,
  getEngineInstance,
  closeEngine,
  getEngineInstanceInfo,
  scanContractHistory,
} from "../core/engine.js";
import {
  gasEstimateForUnprovenTransfer,
  generateTransferProof,
  loadProvider,
  populateProvedTransfer,
  setLoggers,
} from "dop-wallet-v3";
import { parseChain } from "../utils/json.js";
// engine-api.ts

export const engineRouter = express.Router();

// api/engine-api.ts
engineRouter.post("/init", async (req, res) => {
  const {
    engineName,
    dbPath,
    shouldDebug,
    useNativeArtifacts,
    skipMerkletreeScans,
  } = req.body;

  try {
    await initEngine({
      engineName,
      dbPath,
      shouldDebug,
      useNativeArtifacts,
      skipMerkletreeScans,
    });

    res.send({
      message: "Engine initialized successfully",
      dbPath: dbPath || "test.db",
      engineName: engineName || "DOP Engine",
      debug: shouldDebug ?? false,
      nativeArtifacts: useNativeArtifacts ?? false,
      skipMerkletreeScans: skipMerkletreeScans ?? false,
    });
  } catch (err) {
    console.error("Engine init failed:", err);
    res.status(500).send("Failed to initialize engine");
  }
});

engineRouter.get("/engine", (req, res) => {
  const info = getEngineInstanceInfo();
  if (!info) {
    res.status(404).send("Engine Not Initialized");
  } else {
    res.json(info);
  }
});

engineRouter.get("/close", async (req, res) => {
  try {
    await closeEngine();
    res.send("Engine Closed");
  } catch (err) {
    res.status(500).send("Failed to close engine");
  }
});

engineRouter.post("/set-loggers", (req, res) => {
  try {
    const log = (...args: any[]) => console.log("[DOP]", ...args);
    const error = (...args: any[]) => console.error("[DOP ERROR]", ...args);
    setLoggers(log, error);
    res.send("Loggers set");
  } catch (err) {
    res.status(500).send("Failed to set loggers");
  }
});

engineRouter.post("/load-provider", async (req, res) => {
  const { config, network, pollingInterval } = req.body;
  console.log(
    "Loading provider with config:",
    config,
    "network:",
    network,
    "pollingInterval:",
    pollingInterval
  );
  try {
    const response = await loadProvider(config, network, pollingInterval);
    console.log("Provider loaded successfully:", response);
    res.json(response);
  } catch (err) {
    console.error("Failed to load provider:", err);
    res.status(500).send("Failed to load provider");
  }
});

engineRouter.post("/gas-estimate-unproven", async (req, res) => {
  try {
    console.log("Gas estimate request:", req.body);
    const result = await gasEstimateForUnprovenTransfer(
      req.body.txidVersion,
      req.body.network,
      req.body.walletId,
      req.body.encryptionKey,
      req.body.memoText,
      req.body.erc20AmountRecipients,
      req.body.nftAmountRecipients,
      req.body.transactionGasDetailsSerialized,
      req.body.feeTokenDetails,
      req.body.sendWithPublicWallet
    );
    res.json(result);
  } catch (err) {
    console.error("Failed to estimate gas:", err);
    res.status(500).send("Gas estimation failed");
  }
});

engineRouter.post("/generate-transfer-proof", async (req, res) => {
  try {
    const result = await generateTransferProof(
      req.body.txidVersion,
      req.body.network,
      req.body.walletId,
      req.body.encryptionKey,
      req.body.showSenderAddressToRecipient,
      req.body.memo,
      req.body.erc20AmountRecipients,
      req.body.nftAmountRecipients,
      req.body.broadcasterFeeERC20AmountRecipient,
      req.body.sendWithPublicWallet,
      req.body.overallBatchMinGasPrice,
      () => {}
    );

    res.json(result);
  } catch (err) {
    console.error("generateTransferProof failed:", err);
    res.status(500).send("Failed to generate transfer proof");
  }
});

engineRouter.post("/populate-transfer", async (req, res) => {
  const {
    txidVersion,
    network,
    walletId,
    showSenderAddressToRecipient,
    memo,
    tokenAmountRecipients,
    nftAmountRecipients,
    relayerFeeERC20AmountRecipient,
    sendWithPublicWallet,
    overallBatchMinGasPrice,
    gasDetails,
  } = req.body;
  console.log("Populate transfer request:", req.body);
  try {
    const tx = await populateProvedTransfer(
      txidVersion,
      network,
      walletId,
      showSenderAddressToRecipient,
      memo,
      tokenAmountRecipients,
      nftAmountRecipients,
      relayerFeeERC20AmountRecipient,
      sendWithPublicWallet,
      BigInt(overallBatchMinGasPrice),
      gasDetails
    );
    res.json(tx);
  } catch (err) {
    console.error("populateProvedTransfer error:", err);
    res.status(500).send("Failed to populate proved transfer");
  }
});

engineRouter.post("/scan-contract-history", async (req, res) => {
  try {
    const { chain, walletIdFilter } = req.body;
    if (!chain) {
      res.status(400).send("Missing chain");
      return;
    }

    await scanContractHistory(parseChain(chain), walletIdFilter);
    res.sendStatus(204);
    console.log("Contract history scanned successfully");
  } catch (err) {
    console.error("❌ Failed to scan contract history:", err);
    res.status(500).send("Failed to scan contract history");
  }
});
