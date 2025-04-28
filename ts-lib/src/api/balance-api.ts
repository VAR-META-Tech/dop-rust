import express from "express";
import {
  refreshBalances,
  rescanFullUTXOMerkletreesAndWallets,
  resetFullTXIDMerkletreesV2,
} from "dop-wallet-v3";
import { parseChain } from "../utils/json.js"; // your helper

export const balanceRouter = express.Router();

// POST /wallet/refresh-balances
balanceRouter.post("/refresh-balances", async (req, res) => {
  try {
    console.log("Refreshing balances...");
    const { chain, walletIdFilter } = req.body;
    if (!chain) {
      res.status(400).send("Missing chain");
      return;
    }
    await refreshBalances(parseChain(chain), walletIdFilter);
    res.sendStatus(204);
    console.log("Balances refreshed successfully");
  } catch (err) {
    console.log("❌ Failed to refresh balances:", err);
    console.error("❌ Failed to refresh balances:", err);
    res.status(500).send("Failed to refresh balances");
  }
});

// POST /wallet/rescan-full-utxo-merkletrees
balanceRouter.post("/rescan-full-utxo-merkletrees", async (req, res) => {
  try {
    const { chain, walletIdFilter } = req.body;
    if (!chain) {
      res.status(400).send("Missing chain");
      return;
    }
    await rescanFullUTXOMerkletreesAndWallets(
      parseChain(chain),
      walletIdFilter
    );
    res.sendStatus(204);
  } catch (err) {
    console.error("❌ Failed to rescan:", err);
    res.status(500).send("Failed to rescan merkletrees and wallets");
  }
});

// POST /wallet/reset-full-txid-merkletrees
balanceRouter.post("/reset-full-txid-merkletrees", async (req, res) => {
  try {
    const { chain } = req.body;
    if (!chain) {
      res.status(400).send("Missing chain");
      return;
    }
    await resetFullTXIDMerkletreesV2(parseChain(chain));
    res.sendStatus(204);
  } catch (err) {
    console.error("❌ Failed to reset TXID merkletrees:", err);
    res.status(500).send("Failed to reset TXID merkletrees");
  }
});
