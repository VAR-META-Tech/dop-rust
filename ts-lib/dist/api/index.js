import express from "express";
import { engineRouter } from "./engine-api.js";
import { walletRouter } from "./wallet-api.js";
import { balanceRouter } from "./balance-api.js";
import { callbackRouter } from "./callback.js";
import { txEncyptRouter } from "./tx-encrypt-api.js";
import { txTransferRouter } from "./tx-transfer.js";
export const app = express();
app.use(express.json());
app.get("/health", (req, res) => {
    res.send("OK");
});
// Combine all APIs
app.use(engineRouter);
app.use(walletRouter);
app.use(balanceRouter);
app.use(callbackRouter);
app.use(txEncyptRouter);
app.use(txTransferRouter);
