export const stringifyBigInt = (obj) => {
    return JSON.parse(JSON.stringify(obj, (_, v) => (typeof v === 'bigint' ? v.toString() : v)));
};
export const extractWalletInfo = (wallet) => {
    return {
        id: wallet.id,
        dopAddress: wallet.dopAddress,
        spendingPublicKey: wallet.spendingPublicKey.map((v) => v.toString()),
        nullifyingKey: wallet.nullifyingKey.toString(),
        masterPublicKey: wallet.masterPublicKey.toString(),
        creationBlockNumbers: wallet.creationBlockNumbers, // or clean if needed
        viewingKeyPair: {
            privateKey: Array.from(wallet.viewingKeyPair.privateKey),
            pubkey: Array.from(wallet.viewingKeyPair.pubkey),
        },
        merkletrees: wallet.merkletrees || [],
        cachedReceiveCommitments: wallet.cachedReceiveCommitments || [],
        cachedSendCommitments: wallet.cachedSendCommitments || [],
    };
};
