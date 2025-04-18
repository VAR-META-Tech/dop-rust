import express from 'express';
import {
    initEngine,
    getEngineInstance,
    closeEngine,
    getEngineInstanceInfo,
} from '../core/engine.js';

export const engineRouter = express.Router();

// api/engine-api.ts
engineRouter.post('/init', (req, res) => {
    const {
        engineName,
        dbPath,
        shouldDebug,
        useNativeArtifacts,
        skipMerkletreeScans,
    } = req.body;

    try {
        initEngine({
            engineName,
            dbPath,
            shouldDebug,
            useNativeArtifacts,
            skipMerkletreeScans,
        });

        res.send({
            message: 'Engine initialized successfully',
            dbPath: dbPath || 'test.db',
            engineName: engineName || 'DOP Engine',
            debug: shouldDebug ?? false,
            nativeArtifacts: useNativeArtifacts ?? false,
            skipMerkletreeScans: skipMerkletreeScans ?? false,
        });
    } catch (err) {
        console.error('Engine init failed:', err);
        res.status(500).send('Failed to initialize engine');
    }
});


engineRouter.get('/engine', (req, res) => {
    const info = getEngineInstanceInfo();
    if (!info) {
        res.status(404).send('Engine Not Initialized');
    } else {
        res.json(info);
    }
});

engineRouter.get('/close', async (req, res) => {
    try {
        await closeEngine();
        res.send('Engine Closed');
    } catch (err) {
        res.status(500).send('Failed to close engine');
    }
});
