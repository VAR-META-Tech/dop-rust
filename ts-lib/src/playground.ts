import { initEngine, closeEngine, getEngineInstanceInfo } from './core/engine.js';
import { createWallet, getWalletById } from './core/wallet.js';

const mnemonic = 'pause crystal tornado alcohol genre cement fade large song like bag where';
const encryptionKey = '0101010101010101010101010101010101010101010101010101010101010101';

(async () => {
    console.log('Initializing Engine...');
    initEngine();

    const walletInfo = await createWallet(mnemonic, encryptionKey);

    const wallet = getWalletById(walletInfo.id);
    console.log('Full Wallet:', wallet, "end");

    await closeEngine();
})();
