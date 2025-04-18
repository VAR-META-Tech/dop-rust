import express from 'express';
import { createWallet, generateMnemonic, getWalletById } from '../core/wallet.js';
import { extractWalletInfo } from '../utils/json.js';


export const walletRouter = express.Router();

walletRouter.get('/mnemonic', (req, res) => {
    const words = parseInt(req.query.words as string);
    const mnemonic = generateMnemonic(words === 24 ? 24 : 12);
    res.json({ mnemonic });
});


walletRouter.post('/wallet', async (req, res) => {
    const { mnemonic, encryptionKey, creationBlockNumbers } = req.body;
    try {
      const walletInfo = await createWallet(mnemonic, encryptionKey, creationBlockNumbers);
      res.json(walletInfo);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error';
      res.status(500).send(errorMessage);
    }
  });



walletRouter.get('/wallet/:id', (req, res) => {
    const wallet = getWalletById(req.params.id);
    if (!wallet) {
        res.status(404).send('Wallet not found');
    } else {
        res.json(extractWalletInfo(wallet));
    }
});


