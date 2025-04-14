import express from 'express';
import { engineRouter } from './engine-api.js';
import { walletRouter } from './wallet-api.js';
export const app = express();
app.use(express.json());
// Combine all APIs
app.use(engineRouter);
app.use(walletRouter);
