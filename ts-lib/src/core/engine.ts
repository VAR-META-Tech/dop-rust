import LevelDOWN from 'leveldown';
import fs from 'fs';
import {
    ArtifactStore,
    getEngine,
    startDopEngine,
    stopDopEngine,
} from 'dop-wallet-stagging';

const ENGINE_TEST_DB = 'test.db';
const db = new LevelDOWN(ENGINE_TEST_DB);

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

export const initEngine = (useNativeArtifacts = false) => {
    console.log(`Initializing DOP engine with db: ${ENGINE_TEST_DB}`);
    const shouldDebug = false;
    startDopEngine(
        'test engine',
        db,
        shouldDebug,
        artifactStore,
        useNativeArtifacts,
        false,
    );
    console.log('DOP engine initialized');
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


