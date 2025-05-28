import LevelDOWN from "leveldown";
import fs from "fs";
import { ArtifactStore, getEngine, startDopEngine, stopDopEngine, } from "dop-wallet-v3";
const fileExists = async (path) => {
    try {
        await fs.promises.access(path);
        return true;
    }
    catch {
        return false;
    }
};
const artifactStore = new ArtifactStore(fs.promises.readFile, async (dir, path, data) => {
    await fs.promises.mkdir(dir, { recursive: true });
    await fs.promises.writeFile(path, data);
}, fileExists);
// core/engine.ts
export const initEngine = async ({ engineName = "DOP Engine", dbPath = "database/DOP.db", shouldDebug = false, useNativeArtifacts = false, skipMerkletreeScans = false, }) => {
    const db = new LevelDOWN(dbPath);
    return await startDopEngine(engineName, db, shouldDebug, artifactStore, useNativeArtifacts, skipMerkletreeScans);
};
export const getEngineInstance = () => getEngine();
export const getEngineInstanceInfo = () => {
    const engine = getEngine();
    if (!engine)
        return null;
    return {
        wallets: Object.keys(engine?.wallets || {}),
        deploymentBlocks: engine?.deploymentBlocks,
        dopSmartWalletContracts: engine?.dopSmartWalletContracts,
    };
};
export const closeEngine = async () => {
    await stopDopEngine();
};
export async function scanContractHistory(chain, walletIdFilter) {
    const engine = getEngine();
    await engine.scanContractHistory(chain, walletIdFilter);
}
