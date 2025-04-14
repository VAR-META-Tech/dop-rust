import express from 'express';
import { initEngine, getEngineInstance, closeEngine, getEngineInstanceInfo, } from './engine.js';
import { createWallet, getWalletById, } from './wallet.js';
const app = express();
app.use(express.json()); // Needed for parsing POST body
// Engine Routes
app.get('/init', (req, res) => {
    try {
        initEngine();
        res.send('Engine Initialized');
    }
    catch (err) {
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
    }
    else {
        res.json(info);
    }
});
app.get('/close', async (req, res) => {
    try {
        await closeEngine();
        res.send('Engine Closed');
    }
    catch (err) {
        res.status(500).send('Failed to close engine');
    }
});
// Wallet Routes
app.post('/wallet', async (req, res) => {
    const { mnemonic, encryptionKey } = req.body;
    try {
        const walletInfo = await createWallet(mnemonic, encryptionKey);
        res.json(walletInfo);
    }
    catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'An unknown error occurred';
        res.status(500).send(errorMessage);
    }
});
app.get('/wallet/:id', (req, res) => {
    const wallet = getWalletById(req.params.id);
    if (!wallet) {
        res.status(404).send('Wallet not found');
    }
    else {
        res.json(wallet);
    }
});
export { app };
