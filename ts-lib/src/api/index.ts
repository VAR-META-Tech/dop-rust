import express from 'express';
import { engineRouter } from './engine-api.js';
import { walletRouter } from './wallet-api.js';
import { balanceRouter } from './balance-api.js';

export const app = express();

app.use(express.json());

app.get('/health', (req, res) => {
    res.send('OK');
  });
  

// Combine all APIs
app.use(engineRouter);
app.use(walletRouter);
app.use(balanceRouter);
