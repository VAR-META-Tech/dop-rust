import { initEngine, closeEngine, getEngineInstanceInfo } from './core/engine.js';
import { createWallet, getWalletById } from './core/wallet.js';
import { Mnemonic, randomBytes } from 'ethers';
const encryptionKey = '0101010101010101010101010101010101010101010101010101010101010101';
(async () => {
    console.log('🔧 Initializing DOP Engine...');
    try {
        initEngine({
            engineName: 'DOP Engine',
            dbPath: 'DOP.db',
            shouldDebug: false,
            useNativeArtifacts: true,
            skipMerkletreeScans: false,
        });
        // 🧠 Generate a secure 12-word mnemonic
        const mnemonic = Mnemonic.fromEntropy(randomBytes(16)).phrase.trim();
        console.log('🧠 Mnemonic:', mnemonic);
        // 💼 Create new wallet
        const walletInfo = await createWallet(mnemonic, encryptionKey);
        console.log('💼 Wallet Info:', walletInfo);
        // 🔍 Load full wallet details
        const wallet = getWalletById(walletInfo.id);
        console.log('🔍 Full Wallet:', wallet);
        // ⚙️ Get engine internals
        const engineInfo = getEngineInstanceInfo();
        console.log('⚙️ Engine Info:', engineInfo);
    }
    catch (err) {
        console.error('❌ Playground Error:', err);
    }
    finally {
        await closeEngine();
        console.log('🛑 Engine closed.');
    }
})();
