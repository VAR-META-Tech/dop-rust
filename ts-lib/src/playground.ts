import { createDopWallet, createViewOnlyDopWallet, fullWalletForID, gasEstimateForUnprovenTransfer, generateTransferProof, getWalletShareableViewingKey, loadProvider } from 'dop-wallet-stagging';
import { initEngine, closeEngine, getEngineInstanceInfo } from './core/engine.js';
import { createWallet, getWalletById } from './core/wallet.js';
import { FallbackProvider, Mnemonic, randomBytes } from 'ethers';
import { calculateGasLimit, EVMGasType, isDefined, NetworkName, NFTTokenType, TransactionGasDetailsType2 } from 'dop-sharedmodel';
export const MOCK_FALLBACK_PROVIDER_JSON_CONFIG = {
  chainId: 137,
  providers: [
    {
      provider: 'https://light-serene-feather.matic.quiknode.pro/f0cdd8c4c146e68ce2f935bba399ca66cbe45868/',
      priority: 1,
      weight: 2,
      maxLogsPerBatch: 10,
      stallTimeout: 2500,
    },
    {
      provider: 'https://polygon-bor.publicnode.com',
      priority: 1,
      weight: 2,
      maxLogsPerBatch: 10,
      stallTimeout: 2500,
    },
    {
      provider: 'https://light-serene-feather.matic.quiknode.pro/f0cdd8c4c146e68ce2f935bba399ca66cbe45868/',
      priority: 2,
      weight: 2,
      maxLogsPerBatch: 10,
    },
  ],
};
export const MOCK_TOKEN_ADDRESS = '0x5FbDB2315678afecb367f032d93F642f64180aa3';
export const MOCK_TOKEN_ADDRESS_2 =
  '0xe76C6c83af64e4C60245D8C7dE953DF673a7A33D';

export const MOCK_TOKEN_AMOUNTS = [
  {
    tokenAddress: MOCK_TOKEN_ADDRESS,
    amount: BigInt(0x100),
  },
  {
    tokenAddress: MOCK_TOKEN_ADDRESS_2,
    amount: BigInt(0x200),
  },
];
export const MOCK_MNEMONIC =
  'test test test test test test test test test test test junk';

  export const MOCK_DOP_WALLET_ADDRESS =
  '0zk1q8hxknrs97q8pjxaagwthzc0df99rzmhl2xnlxmgv9akv32sua0kfrv7j6fe3z53llhxknrs97q8pjxaagwthzc0df99rzmhl2xnlxmgv9akv32sua0kg0zpzts';
  const MOCK_TOKEN_AMOUNT_RECIPIENTS =
  MOCK_TOKEN_AMOUNTS.map(erc20Amount => ({
    ...erc20Amount,
    recipientAddress: MOCK_DOP_WALLET_ADDRESS,
  }));
export const MOCK_MEMO =
  'A nice little mock memo, and how bout a little more for ya? 🤩';
const encryptionKey =
  '0101010101010101010101010101010101010101010101010101010101010101';
  export const MOCK_DB_ENCRYPTION_KEY =
  '0101010101010101010101010101010101010101010101010101010101010101';
  export const MOCK_FEE_TOKEN_DETAILS = {
    tokenAddress: MOCK_TOKEN_ADDRESS,
    feePerUnitGas: BigInt('0x2000000000000000000'), // 2x
  };
  export const MOCK_TRANSACTION_GAS_DETAILS_SERIALIZED_TYPE_2: TransactionGasDetailsType2 =
  {
    evmGasType: EVMGasType.Type2,
    gasEstimate: 0n,
    maxFeePerGas: BigInt('0x1234567890'),
    maxPriorityFeePerGas: BigInt('0x123456'),
  };
  export const MOCK_NFT_ADDRESS = '0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d';
  export const MOCK_NFT_AMOUNTS = [
    {
      nftAddress: MOCK_NFT_ADDRESS,
      nftTokenType: NFTTokenType.ERC721,
      tokenSubID: '0x01',
      amount: BigInt(0x01),
    },
    {
      nftAddress: MOCK_NFT_ADDRESS,
      nftTokenType: NFTTokenType.ERC1155,
      tokenSubID: '0x02',
      amount: BigInt(0x02),
    },
  ];

  export const MOCK_NFT_AMOUNT_RECIPIENTS =
  MOCK_NFT_AMOUNTS.map(nftAmount => ({
    ...nftAmount,
    recipientAddress: MOCK_DOP_WALLET_ADDRESS,
  }));


  export const MOCK_TOKEN_FEE = {
    tokenAddress: MOCK_TOKEN_ADDRESS,
    amount: BigInt(0x0300),
  };
  const overallBatchMinGasPrice = BigInt('0x1000');

(async () => {
  console.log('🔧 Initializing DOP Engine...');
  try {
    initEngine({
      engineName: 'DOP Engine',
      dbPath: 'database/DOP.db',
      shouldDebug: false,
      useNativeArtifacts: true,
      skipMerkletreeScans: false,
    });

    const dopWalletInfo = await createDopWallet(
      MOCK_DB_ENCRYPTION_KEY,
      MOCK_MNEMONIC,
      undefined, // creationBlockNumbers
    );
    if (!isDefined(dopWalletInfo)) {
      throw new Error('Expected dopWalletInfo');
    }
    let dopWallet = fullWalletForID(dopWalletInfo.id);

    const relayerWalletInfo = await createDopWallet(
      MOCK_DB_ENCRYPTION_KEY,
      MOCK_MNEMONIC,
      undefined, // creationBlockNumbers
    );
    if (!isDefined(relayerWalletInfo)) {
      throw new Error('Expected relayerWalletInfo');
    }

    const relayerDopAddress = relayerWalletInfo.dopAddress;

    let relayerFeeERC20AmountRecipient = {
      ...MOCK_TOKEN_FEE,
      recipientAddress: relayerDopAddress,
    };


    const res = await generateTransferProof(
      NetworkName.Polygon,
      dopWallet.id,
      MOCK_DB_ENCRYPTION_KEY,
      true, // showSenderAddressToRecipient
      MOCK_MEMO,
      MOCK_TOKEN_AMOUNT_RECIPIENTS,
      MOCK_NFT_AMOUNT_RECIPIENTS,
      relayerFeeERC20AmountRecipient,
      false, // sendWithPublicWallet
      overallBatchMinGasPrice,
      () => {}, // progressCallback
    );
      console.log(res);
  } catch (err) {
    console.error('❌ Playground Error:', err);
  } finally {
    await closeEngine();
    console.log('🛑 Engine closed.');
  }
})();
