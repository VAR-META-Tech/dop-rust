import express from 'express';

import {getRandomBytes} from '../core/encrypts.js';
import { hashPasswordString, setEncryptionKeyFromPassword, getEncryptionKeyFromPassword } from '../core/encrypts.js';

export const encryptRouter = express.Router();

encryptRouter.get('/getRandomBytes', async (req, res) => {
    const { length } = req.query;
    try {
        const bytes = getRandomBytes(10);
        res.json({ bytes });
    } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Can not get random bytes';
        res.status(500).send(errorMessage);
    }
});

encryptRouter.get('/hashpwdstr', async (req, res) => {
    const { password, salt, iterations } = req.query;
    try {
        const hs = await hashPasswordString(
            password as string || '',
            salt as string || '',
            parseInt(iterations as string || '0')
        );
        res.json({ hs });
    } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Can not get hash password string';
        res.status(500).send(errorMessage);
    }
});

encryptRouter.get('/setEncryptionKeyFromPassword', async (req, res) => {
    const { password } = req.query;
    try {
        const encryptionKey = await setEncryptionKeyFromPassword(password as string || '');
        res.json({ encryptionKey });
    } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Can not set encryption key from password';
        res.status(500).send(errorMessage);
    }
});

encryptRouter.get('/getEncryptionKeyFromPassword', async (req, res) => {
    const { password } = req.query;
    try {
        const encryptionKey = await getEncryptionKeyFromPassword(password as string || '');
        res.json({ encryptionKey });
    } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Can not get encryption key from password';
        res.status(500).send(errorMessage);
    }
})