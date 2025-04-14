import { createDopWallet, fullWalletForID, } from 'dop-wallet-stagging';
export const createWallet = async (mnemonic, encryptionKey) => {
    const walletInfo = await createDopWallet(encryptionKey, mnemonic, { Ethereum: 0, Polygon: 2 });
    if (!walletInfo) {
        throw new Error('Failed to create wallet');
    }
    return walletInfo;
};
export const getWalletById = (id) => {
    return fullWalletForID(id);
};
