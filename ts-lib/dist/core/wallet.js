import { createDopWallet, createViewOnlyDopWallet, fullWalletForID, getWalletShareableViewingKey } from 'dop-wallet-v3';
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
export const getWalletShareableViewingKeyById = async (id) => {
    return await getWalletShareableViewingKey(id);
};
export const createViewOnlyWallet = (encryptionKey, shareableViewingKey, creationBlockNumbers) => {
    return createViewOnlyDopWallet(encryptionKey, shareableViewingKey, creationBlockNumbers);
};
