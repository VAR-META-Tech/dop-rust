import express from "express";
import { initEngine, closeEngine, getEngineInstanceInfo, scanContractHistory, } from "../core/engine.js";
import { loadProvider, setLoggers, } from "dop-wallet-v3";
import { parseChain } from "../utils/json.js";
// engine-api.ts
export const engineRouter = express.Router();
// api/engine-api.ts
engineRouter.post("/init", async (req, res) => {
    const { engineName, dbPath, shouldDebug, useNativeArtifacts, skipMerkletreeScans, } = req.body;
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
    }
    catch (err) {
        console.error("Engine init failed:", err);
        res.status(500).send("Failed to initialize engine");
    }
});
engineRouter.get("/engine", (req, res) => {
    const info = getEngineInstanceInfo();
    if (!info) {
        res.status(404).send("Engine Not Initialized");
    }
    else {
        res.json(info);
    }
});
engineRouter.get("/close", async (req, res) => {
    try {
        await closeEngine();
        res.send("Engine Closed");
    }
    catch (err) {
        res.status(500).send("Failed to close engine");
    }
});
engineRouter.post("/set-loggers", (req, res) => {
    try {
        const log = (...args) => console.log("[DOP]", ...args);
        const error = (...args) => console.error("[DOP ERROR]", ...args);
        setLoggers(log, error);
        res.send("Loggers set");
    }
    catch (err) {
        res.status(500).send("Failed to set loggers");
    }
});
engineRouter.post("/load-provider", async (req, res) => {
    const { config, network, pollingInterval } = req.body;
    try {
        const response = await loadProvider(config, network, pollingInterval);
        res.json(response);
    }
    catch (err) {
        console.error("Failed to load provider:", err);
        res.status(500).send("Failed to load provider");
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
    }
    catch (err) {
        console.error("❌ Failed to scan contract history:", err);
        res.status(500).send("Failed to scan contract history");
    }
});
