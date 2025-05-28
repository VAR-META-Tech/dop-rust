import express from "express";
import { createViewOnlyWallet, createWallet, generateMnemonic, getWalletById, getWalletShareableViewingKeyById, } from "../core/wallet.js";
import { extractWalletInfo, parseChain } from "../utils/json.js";
import { assertValidDopAddress, assertValidEthAddress, awaitMultipleWalletScans, awaitWalletScan, deleteWalletByID, getDopAddress, getDopWalletAddressData, getDopWalletPrivateViewingKey, getWalletMnemonic, loadWalletByID, signWithWalletViewingKey, unloadWalletByID, validateDopAddress, validateEthAddress, } from "dop-wallet-v3";
import { toUtf8Bytes } from "ethers";
export const walletRouter = express.Router();
walletRouter.get("/mnemonic", (req, res) => {
    const words = parseInt(req.query.words);
    const mnemonic = generateMnemonic(words === 24 ? 24 : 12);
    res.json({ mnemonic });
});
walletRouter.post("/wallet", async (req, res) => {
    const { mnemonic, encryptionKey, creationBlockNumbers } = req.body;
    try {
        const walletInfo = await createWallet(mnemonic, encryptionKey, creationBlockNumbers);
        res.json(walletInfo);
    }
    catch (err) {
        const errorMessage = err instanceof Error ? err.message : "Unknown error";
        res.status(500).send(errorMessage);
    }
});
walletRouter.post("/wallet/view-only", async (req, res) => {
    const { encryptionKey, shareableViewingKey, creationBlockNumbers } = req.body;
    try {
        const walletInfo = await createViewOnlyWallet(encryptionKey, shareableViewingKey, creationBlockNumbers);
        res.json(walletInfo);
    }
    catch (err) {
        res.status(500).send("Failed to create view-only wallet");
    }
});
walletRouter.get("/wallet/:id/mnemonic", async (req, res) => {
    const { id } = req.params;
    const { encryptionKey } = req.query;
    try {
        const mnemonic = await getWalletMnemonic(encryptionKey, id);
        res.json({ mnemonic });
    }
    catch (err) {
        res.status(500).send("Failed to retrieve mnemonic");
    }
});
walletRouter.post("/wallet/sign-message", async (req, res) => {
    const { walletId, message } = req.body;
    try {
        if (typeof message !== "string" || !message.length) {
            res.status(400).send("Message must be a non-empty string");
            return;
        }
        const hexMessage = "0x" + Buffer.from(toUtf8Bytes(message)).toString("hex");
        const signature = await signWithWalletViewingKey(walletId, hexMessage);
        res.json({ signature });
    }
    catch (err) {
        res.status(500).send("Failed to sign message");
    }
});
walletRouter.post("/wallet/load", async (req, res) => {
    const { encryptionKey, dopWalletID, isViewOnlyWallet } = req.body;
    try {
        const walletInfo = await loadWalletByID(encryptionKey, dopWalletID, isViewOnlyWallet);
        res.json(walletInfo);
    }
    catch (err) {
        res.status(500).send("Failed to load wallet");
    }
});
walletRouter.get("/wallet/:id/scan", async (req, res) => {
    const { id } = req.params;
    try {
        const chain = parseChain(req.query.chain);
        const result = await awaitWalletScan(id, chain);
        res.json({ result });
    }
    catch (err) {
        res.status(400).send("Invalid chain object or failed to scan wallet");
    }
});
walletRouter.get("/wallet/:id/scan-multiple", async (req, res) => {
    const { id } = req.params;
    try {
        const chain = parseChain(req.query.chain);
        const count = parseInt(req.query.count);
        await awaitMultipleWalletScans(id, chain, count);
        res.sendStatus(204);
    }
    catch (err) {
        res
            .status(400)
            .send("Invalid input or failed to scan wallet multiple times");
    }
});
// Unload wallet
walletRouter.get("/wallet/:id/unload", (req, res) => {
    const { id } = req.params;
    try {
        unloadWalletByID(id);
        res.sendStatus(204);
    }
    catch (err) {
        res.status(500).send("Failed to unload wallet");
    }
});
// Delete wallet
walletRouter.delete("/wallet/:id/delete", async (req, res) => {
    const { id } = req.params;
    try {
        await deleteWalletByID(id);
        res.sendStatus(204);
    }
    catch (err) {
        res.status(500).send("Failed to delete wallet");
    }
});
// Get wallet address data
walletRouter.get("/wallet/address-data", (req, res) => {
    const { address } = req.query;
    try {
        if (typeof address !== "string") {
            res.status(400).send("Invalid address format");
            return;
        }
        const data = getDopWalletAddressData(address);
        res.json({
            masterPublicKey: data.masterPublicKey?.toString?.() ?? null,
            viewingPublicKey: Array.from(data.viewingPublicKey || []),
            version: data.version,
            chain: data.chain ?? null,
        });
    }
    catch (err) {
        res.status(500).send("Failed to get address data");
    }
});
// Get private viewing key
walletRouter.get("/wallet/:id/private-viewing-key", (req, res) => {
    const { id } = req.params;
    try {
        const key = getDopWalletPrivateViewingKey(id);
        res.json({ privateViewingKey: Buffer.from(key).toString("hex") });
    }
    catch (err) {
        res.status(500).send("Failed to get private viewing key");
    }
});
// Get DOP address
walletRouter.get("/wallet/:id/dop-address", (req, res) => {
    const { id } = req.params;
    try {
        const dopAddress = getDopAddress(id);
        res.json({ dopAddress });
    }
    catch (err) {
        res.status(500).send("Failed to get DOP address");
    }
});
// Validate DOP address
walletRouter.get("/validate/dop-address", (req, res) => {
    const { address } = req.query;
    try {
        const valid = validateDopAddress(address);
        res.json({ valid });
    }
    catch (err) {
        res.status(500).send("DOP address validation failed");
    }
});
// Validate ETH address
walletRouter.get("/validate/eth-address", (req, res) => {
    const { address } = req.query;
    try {
        const valid = validateEthAddress(address);
        res.json({ valid });
    }
    catch (err) {
        res.status(500).send("ETH address validation failed");
    }
});
// Assert ETH address (throws on invalid)
walletRouter.get("/assert/eth-address", (req, res) => {
    const { address } = req.query;
    try {
        assertValidEthAddress(address); // Throws if invalid
        res.sendStatus(204); // No content = valid
    }
    catch (err) {
        res.status(400).send("Invalid ETH address format");
    }
});
// Assert ETH address (throws on invalid)
walletRouter.get("/assert/dop-address", (req, res) => {
    const { address } = req.query;
    try {
        assertValidDopAddress(address); // Throws if invalid
        res.sendStatus(204); // No content = valid
    }
    catch (err) {
        res.status(400).send("Invalid DOP address format");
    }
});
walletRouter.get("/wallet/:id", (req, res) => {
    const wallet = getWalletById(req.params.id);
    if (!wallet) {
        res.status(404).send("Wallet not found");
    }
    else {
        res.json(extractWalletInfo(wallet));
    }
});
walletRouter.get("/wallet/:id/shareable-viewing-key", async (req, res) => {
    try {
        const key = await getWalletShareableViewingKeyById(req.params.id);
        res.json({ shareableViewingKey: key });
    }
    catch (err) {
        res.status(500).send("Failed to get shareable viewing key");
    }
});
