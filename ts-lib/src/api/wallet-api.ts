import express from 'express';
import { createViewOnlyWallet, createWallet, generateMnemonic, getWalletById, getWalletShareableViewingKeyById } from '../core/wallet.js';
import { extractWalletInfo } from '../utils/json.js';
import { getWalletMnemonic, loadWalletByID, signWithWalletViewingKey } from 'dop-wallet-v3';

import {  toUtf8Bytes } from "ethers";

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

walletRouter.get('/wallet/:id/shareable-viewing-key', async (req, res) => {
  try {
    const key = await  getWalletShareableViewingKeyById(req.params.id);
    res.json({ shareableViewingKey: key });
  } catch (err) {
    res.status(500).send('Failed to get shareable viewing key');
  }
});

walletRouter.post('/wallet/view-only', async (req, res) => {
  const { encryptionKey, shareableViewingKey, creationBlockNumbers } = req.body;

  try {
    const walletInfo = await createViewOnlyWallet(
      encryptionKey,
      shareableViewingKey,
      creationBlockNumbers
    );
    res.json(walletInfo);
  } catch (err) {
    res.status(500).send('Failed to create view-only wallet');
  }
});

walletRouter.get('/wallet/:id/mnemonic', async (req, res) => {
  const { id } = req.params;
  const { encryptionKey } = req.query;
  try {
    const mnemonic = await getWalletMnemonic(encryptionKey as string, id);
    res.json({ mnemonic });
  } catch (err) {
    res.status(500).send('Failed to retrieve mnemonic');
  }
});

walletRouter.post('/wallet/sign-message', async (req, res) => {
  const { walletId, message } = req.body;

  try {
    if (typeof message !== 'string' || !message.length) {
       res.status(400).send('Message must be a non-empty string');
       return;
    }

    const hexMessage = "0x" + Buffer.from(toUtf8Bytes(message)).toString("hex");
    const signature = await signWithWalletViewingKey(walletId, hexMessage);
    res.json({ signature });
  } catch (err) {
    res.status(500).send('Failed to sign message');
  }
});

walletRouter.post('/wallet/load', async (req, res) => {
  const { encryptionKey, dopWalletID, isViewOnlyWallet } = req.body;
  try {
    const walletInfo = await loadWalletByID(encryptionKey, dopWalletID, isViewOnlyWallet);
    res.json(walletInfo);
  } catch (err) {
    res.status(500).send('Failed to load wallet');
  }
});
