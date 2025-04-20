import LevelDOWN from 'leveldown';
import fs from 'fs';
import { ArtifactStore, getEngine, startDopEngine, stopDopEngine } from 'dop-wallet-v3';
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
export const initEngine = ({ engineName = 'DOP Engine', dbPath = 'database/DOP.db', shouldDebug = false, useNativeArtifacts = false, skipMerkletreeScans = false, }) => {
    console.log(`[Engine Init] ${engineName} with DB: ${dbPath}`);
    const db = new LevelDOWN(dbPath);
    startDopEngine(engineName, db, shouldDebug, artifactStore, useNativeArtifacts, skipMerkletreeScans);
    console.log('[Engine Init] DOP engine initialized successfully.');
};
export const getEngineInstance = () => getEngine();
export const getEngineInstanceInfo = () => {
    const engine = getEngine();
    if (!engine)
        return null;
    console.log('Engine instance:', engine);
    return {
        wallets: Object.keys(engine?.wallets || {}),
        deploymentBlocks: engine?.deploymentBlocks,
        dopSmartWalletContracts: engine?.dopSmartWalletContracts,
    };
};
export const closeEngine = async () => {
    await stopDopEngine();
};
