import { createDopWallet, fullWalletForID, } from 'dop-wallet-stagging';
import { Mnemonic, randomBytes } from 'ethers';
export const createWallet = async (mnemonic, encryptionKey, creationBlockNumbers) => {
    const walletInfo = await createDopWallet(encryptionKey, mnemonic, creationBlockNumbers // <-- pass as-is
    );
    if (!walletInfo) {
        throw new Error('Failed to create wallet');
    }
    return walletInfo;
};
export const getWalletById = (id) => {
    return fullWalletForID(id);
};
export const generateMnemonic = (words = 12) => {
    const bits = words === 24 ? 32 : 16;
    return Mnemonic.fromEntropy(randomBytes(bits)).phrase.trim();
};
