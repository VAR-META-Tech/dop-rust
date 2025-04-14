import express from 'express';
import { initEngine, getEngineInstance, closeEngine, getEngineInstanceInfo, } from '../core/engine.js';
export const engineRouter = express.Router();
engineRouter.get('/init', (req, res) => {
    try {
        console.log("init");
        initEngine();
        res.send('Engine Initialized');
    }
    catch (err) {
        res.status(500).send('Failed to init engine');
    }
});
engineRouter.get('/status', (req, res) => {
    const status = getEngineInstance() ? 'READY' : 'NOT_INITIALIZED';
    res.send(status);
});
engineRouter.get('/engine', (req, res) => {
    const info = getEngineInstanceInfo();
    if (!info) {
        res.status(404).send('Engine Not Initialized');
    }
    else {
        res.json(info);
    }
});
engineRouter.get('/close', async (req, res) => {
    try {
        await closeEngine();
        res.send('Engine Closed');
    }
    catch (err) {
        res.status(500).send('Failed to close engine');
    }
});
