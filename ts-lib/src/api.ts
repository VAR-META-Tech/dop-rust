import express from 'express';
import {
    greet,
    initEngine,
    getEngineInstance,
    closeEngine,
    getEngineInstanceInfo,
} from './engine.js';

const app = express();

app.get('/init', (req, res) => {
    try {
        initEngine();
        res.send('Engine Initialized');
    } catch (err) {
        res.status(500).send('Failed to init engine');
    }
});

app.get('/status', (req, res) => {
    const status = getEngineInstance() ? 'READY' : 'NOT_INITIALIZED';
    res.send(status);
});

app.get('/engine', (req, res) => {
    const info = getEngineInstanceInfo();
    if (!info) {
        res.status(404).send('Engine Not Initialized');
    } else {
        res.json(info);
    }
});



app.get('/close', async (req, res) => {
    try {
        await closeEngine();
        res.send('Engine Closed');
    } catch (err) {
        res.status(500).send('Failed to close engine');
    }
});

app.get('/greet/:name', (req, res) => {
    res.send(greet(req.params.name));
});

export { app };
