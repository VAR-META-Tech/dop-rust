import express from "express";
import {
  setOnUTXOMerkletreeScanCallback,
  setOnTXIDMerkletreeScanCallback,
} from "dop-wallet-v3";

export const callbackRouter = express.Router();

// Setup UTXO Scan Callback
callbackRouter.post("/setup-utxo-scan-callback", (req, res) => {
  setOnUTXOMerkletreeScanCallback(async (scanData) => {
    await fetch("http://localhost:4000/utxo-scan-update", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(scanData),
    }).catch(console.error);
  });
  res.sendStatus(204);
});

// Setup TXID Scan Callback
callbackRouter.post("/setup-txid-scan-callback", (req, res) => {
  setOnTXIDMerkletreeScanCallback(async (scanData) => {
    await fetch("http://localhost:4000/txid-scan-update", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(scanData),
    }).catch(console.error);
  });
  res.sendStatus(204);
});
