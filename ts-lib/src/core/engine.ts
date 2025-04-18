import LevelDOWN from 'leveldown';
import fs from 'fs';
import {
    ArtifactStore,
    getEngine,
    startDopEngine,
    stopDopEngine,
} from 'dop-wallet-stagging';



const fileExists = async (path: string) => {
    try {
        await fs.promises.access(path);
        return true;
    } catch {
        return false;
    }
};

const artifactStore = new ArtifactStore(
    fs.promises.readFile,
    async (dir, path, data) => {
        await fs.promises.mkdir(dir, { recursive: true });
        await fs.promises.writeFile(path, data);
    },
    fileExists,
);

// core/engine.ts
export const initEngine = ({
    engineName = 'DOP Engine',
    dbPath = 'DOP.db',
    shouldDebug = false,
    useNativeArtifacts = false,
    skipMerkletreeScans = false,
}: {
    engineName?: string;
    dbPath?: string;
    shouldDebug?: boolean;
    useNativeArtifacts?: boolean;
    skipMerkletreeScans?: boolean;
}) => {
    console.log(`[Engine Init] ${engineName} with DB: ${dbPath}`);
    const db = new LevelDOWN(dbPath);

    startDopEngine(
        engineName,
        db,
        shouldDebug,
        artifactStore,
        useNativeArtifacts,
        skipMerkletreeScans,
    );

    console.log('[Engine Init] DOP engine initialized successfully.');
};


export const getEngineInstance = () => getEngine();

export const getEngineInstanceInfo = () => {
    const engine = getEngine();
    if (!engine) return null;
    console.log('Engine instance:', engine);
    return {
        merkletrees: engine.merkletrees,
        wallets: Object.keys(engine?.wallets || {}),
        deploymentBlocks: engine?.deploymentBlocks,
        dopSmartWalletContracts: engine?.dopSmartWalletContracts,
        relayAdaptContracts: engine?.relayAdaptContracts,
    };
};

export const closeEngine = async () => {
    await stopDopEngine();
};


