import { DopWalletInfo } from 'dop-sharedmodel';
import {
    createDopWallet,
    createViewOnlyDopWallet,
    fullWalletForID,
    getWalletShareableViewingKey,
} from 'dop-wallet-stagging';
import { Mnemonic, randomBytes } from 'ethers';

export const createWallet = async (
    mnemonic: string,
    encryptionKey: string,
    creationBlockNumbers?: Record<string, number>
  ): Promise<DopWalletInfo> => {
    const walletInfo = await createDopWallet(
      encryptionKey,
      mnemonic,
      creationBlockNumbers // <-- pass as-is
    );
  
    if (!walletInfo) {
      throw new Error('Failed to create wallet');
    }
  
    return walletInfo;
  };
  

export const getWalletById = (id: string) => {
    return fullWalletForID(id);
};

export const generateMnemonic = (words: 12 | 24 = 12): string => {
    const bits = words === 24 ? 32 : 16;
    return Mnemonic.fromEntropy(randomBytes(bits)).phrase.trim();
};


export const getWalletShareableViewingKeyById = (id: string) => {
  return getWalletShareableViewingKey(id);
};

export const createViewOnlyWallet = (
  encryptionKey: string,
  shareableViewingKey: string,
  creationBlockNumbers?: Record<string, number>
) => {
  return createViewOnlyDopWallet(
    encryptionKey,
    shareableViewingKey,
    creationBlockNumbers
  );
};
