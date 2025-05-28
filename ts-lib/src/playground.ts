import {
  Chain,
  ChainType,
  DopERC20Amount,
  DopTransactionGasEstimateResponse,
  EVMGasType,
  FallbackProviderJsonConfig,
  isDefined,
  MerkletreeScanStatus,
  MerkletreeScanUpdateEvent,
  NETWORK_CONFIG,
  NetworkName,
  NFTTokenType,
  TransactionGasDetailsType2,
} from "dop-sharedmodels-v3";
import {
  initEngine,
  closeEngine,
  getEngineInstanceInfo,
} from "./core/engine.js";
import { createWallet, getWalletById } from "./core/wallet.js";
import {
  ContractTransaction,
  FallbackProvider,
  Mnemonic,
  Network,
  randomBytes,
  toUtf8Bytes,
} from "ethers";
import {
  awaitWalletScan,
  createDopWallet,
  fullWalletForID,
  getDopWalletAddressData,
  getWalletMnemonic,
  loadProvider,
  signWithWalletViewingKey,
  refreshBalances,
  rescanFullUTXOMerkletreesAndWallets,
  resetFullTXIDMerkletreesV2,
  getEngine,
  setOnUTXOMerkletreeScanCallback,
  setOnTXIDMerkletreeScanCallback,
  assertValidDopAddress,
  assertNotBlockedAddress,
  gasEstimateResponse,
  getGasEstimate,
  gasEstimateForEncryptBaseToken,
  gasEstimateForUnprovenDecrypt,
} from "dop-wallet-v3";
export const MOCK_FALLBACK_PROVIDER_JSON_CONFIG = {
  chainId: 137,
  providers: [
    {
      provider:
        "https://light-serene-feather.matic.quiknode.pro/f0cdd8c4c146e68ce2f935bba399ca66cbe45868/",
      priority: 1,
      weight: 2,
      maxLogsPerBatch: 10,
      stallTimeout: 2500,
    },
    {
      provider: "https://polygon-bor.publicnode.com",
      priority: 1,
      weight: 2,
      maxLogsPerBatch: 10,
      stallTimeout: 2500,
    },
    {
      provider:
        "https://light-serene-feather.matic.quiknode.pro/f0cdd8c4c146e68ce2f935bba399ca66cbe45868/",
      priority: 2,
      weight: 2,
      maxLogsPerBatch: 10,
    },
  ],
};
export const MOCK_TOKEN_ADDRESS = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
export const MOCK_TOKEN_ADDRESS_2 =
  "0xe76C6c83af64e4C60245D8C7dE953DF673a7A33D";

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
  "test test test test test test test test test test test junk";

export const MOCK_DOP_WALLET_ADDRESS =
  "0zk1q8hxknrs97q8pjxaagwthzc0df99rzmhl2xnlxmgv9akv32sua0kfrv7j6fe3z53llhxknrs97q8pjxaagwthzc0df99rzmhl2xnlxmgv9akv32sua0kg0zpzts";
const MOCK_TOKEN_AMOUNT_RECIPIENTS = MOCK_TOKEN_AMOUNTS.map((erc20Amount) => ({
  ...erc20Amount,
  recipientAddress: MOCK_DOP_WALLET_ADDRESS,
}));
export const MOCK_MEMO =
  "A nice little mock memo, and how bout a little more for ya? 🤩";
const encryptionKey =
  "0101010101010101010101010101010101010101010101010101010101010101";
export const MOCK_DB_ENCRYPTION_KEY =
  "0101010101010101010101010101010101010101010101010101010101010101";
export const MOCK_FEE_TOKEN_DETAILS = {
  tokenAddress: MOCK_TOKEN_ADDRESS,
  feePerUnitGas: BigInt("0x2000000000000000000"), // 2x
};
export const MOCK_TRANSACTION_GAS_DETAILS_SERIALIZED_TYPE_2: TransactionGasDetailsType2 =
  {
    evmGasType: EVMGasType.Type2,
    gasEstimate: 0n,
    maxFeePerGas: BigInt("0x1234567890"),
    maxPriorityFeePerGas: BigInt("0x123456"),
  };
export const MOCK_NFT_ADDRESS = "0xbc4ca0eda7647a8ab7c2061c2e118a18a936f13d";
export const MOCK_NFT_AMOUNTS = [
  {
    nftAddress: MOCK_NFT_ADDRESS,
    nftTokenType: NFTTokenType.ERC721,
    tokenSubID: "0x01",
    amount: BigInt(0x01),
  },
  {
    nftAddress: MOCK_NFT_ADDRESS,
    nftTokenType: NFTTokenType.ERC1155,
    tokenSubID: "0x02",
    amount: BigInt(0x02),
  },
];

export const MOCK_NFT_AMOUNT_RECIPIENTS = MOCK_NFT_AMOUNTS.map((nftAmount) => ({
  ...nftAmount,
  recipientAddress: MOCK_DOP_WALLET_ADDRESS,
}));

export const MOCK_TOKEN_FEE = {
  tokenAddress: MOCK_TOKEN_ADDRESS,
  amount: BigInt(0x0300),
};
const networkName = NetworkName.EthereumSepolia;
export const MOCK_FALLBACK_PROVIDER_JSON_CONFIG_SEPOLIA: FallbackProviderJsonConfig =
  {
    chainId: 11155111,
    providers: [
      {
        provider: "https://sepolia.drpc.org",
        priority: 3,
        weight: 3,
        maxLogsPerBatch: 2,
        stallTimeout: 2500,
      },
      {
        provider: "https://ethereum-sepolia-rpc.publicnode.com",
        priority: 3,
        weight: 2,
        maxLogsPerBatch: 5,
      },
    ],
  };
export const MOCK_ETH_WALLET_ADDRESS =
  "0x9E9F988356f46744Ee0374A17a5Fa1a3A3cC3777";
const overallBatchMinGasPrice = BigInt("0x1000");
const loadEngineProvider = async () => {
  const ETH_PROVIDERS_JSON: FallbackProviderJsonConfig = {
    chainId: 1,
    providers: [
      {
        provider: "https://cloudflare-eth.com/",
        priority: 1,
        weight: 1,
      },
      {
        provider: "https://rpc.ankr.com/eth",
        priority: 2,
        weight: 1,
      },
    ],
  };
  const pollingInterval = 1000 * 60 * 5; // 5 min
  try {
    const { feesSerialized } = await loadProvider(
      ETH_PROVIDERS_JSON,
      NetworkName.Ethereum,
      pollingInterval
    );
  } catch (err) {
    console.error("❌ Failed to load provider:", err);
  }
};
declare type Optional<T> = T | undefined;
let currentUTXOMerkletreeScanStatus: Optional<MerkletreeScanStatus>;
let currentTXIDMerkletreeScanStatus: Optional<MerkletreeScanStatus>;

export const utxoMerkletreeHistoryScanCallback = (
  scanData: MerkletreeScanUpdateEvent
): void => {
  console.log("UTXOMerkletree scan data:", scanData);
  currentUTXOMerkletreeScanStatus = scanData.scanStatus;
};

export const txidMerkletreeHistoryScanCallback = (
  scanData: MerkletreeScanUpdateEvent
): void => {
  console.log("TXIDMerkletree scan data:", scanData);
  currentTXIDMerkletreeScanStatus = scanData.scanStatus;
};

import {
  ByteUtils,
  DopEngine,
  EncryptNoteERC20,
  RelayAdaptVersionedSmartContracts,
  TXIDVersion,
} from "dop-engine-v3";

export const isV2Test = (): boolean => {
  return process.env.V2_TEST === "1";
};

export const getTestTXIDVersion = () => {
  if (isV2Test()) {
    return TXIDVersion.V2_PoseidonMerkle;
  }
  return TXIDVersion.V3_PoseidonMerkle;
};

export const initTestEngineNetworks = async (
  networkName = NetworkName.EthereumSepolia,
  mockConfig = MOCK_FALLBACK_PROVIDER_JSON_CONFIG_SEPOLIA
) => {
  // Don't wait for async. It will try to load historical events, which takes a while.
  await loadProvider(
    mockConfig,
    networkName,
    10_000 // pollingInterval
  );
  const { chain } = NETWORK_CONFIG[networkName];
  // eslint-disable-next-line @typescript-eslint/no-floating-promises
  getEngine().scanContractHistory(
    chain,
    undefined // walletIdFilter
  );
};
// const generateEncryptBaseTokenTransaction = async (
//   txidVersion: TXIDVersion,
//   networkName: NetworkName,
//   dopAddress: string,
//   encryptPrivateKey: string,
//   wrappedERC20Amount: DopERC20Amount,
//   fromWalletAddress: string
// ): Promise<any> => {
//   try {
//     const { masterPublicKey, viewingPublicKey } =
//       DopEngine.decodeAddress(dopAddress);
//     const random = ByteUtils.randomHex(16);

//     const { amount, tokenAddress } = wrappedERC20Amount;

//     const encrypt = new EncryptNoteERC20(
//       masterPublicKey,
//       random,
//       amount,
//       tokenAddress
//     );

//     const encryptRequest = await encrypt.serialize(
//       ByteUtils.hexToBytes(encryptPrivateKey),
//       viewingPublicKey
//     );

//     const { chain } = NETWORK_CONFIG[networkName];
//     console.log("Chain:", chain);
//     console.log(txidVersion);
//     const transaction =
//       await RelayAdaptVersionedSmartContracts.populateEncryptBaseToken(
//         txidVersion,
//         chain,
//         encryptRequest,
//         fromWalletAddress
//       );

//     return transaction;
//   } catch (err) {
//     console.error("❌1 generateEncryptBaseTokenTransaction:", err);
//   }
// };
// export const gasEstimateForEncryptBaseToken = async (
//   txidVersion: TXIDVersion,
//   networkName: NetworkName,
//   dopAddress: string,
//   encryptPrivateKey: string,
//   wrappedERC20Amount: DopERC20Amount,
//   fromWalletAddress: string
// ): Promise<any> => {
//   try {
//     assertValidDopAddress(dopAddress);
//     assertNotBlockedAddress(fromWalletAddress);

//     const transaction = await generateEncryptBaseTokenTransaction(
//       txidVersion,
//       networkName,
//       dopAddress,
//       encryptPrivateKey,
//       wrappedERC20Amount,
//       fromWalletAddress
//     );
//     console.log("Transaction:", transaction);
//     const sendWithPublicWallet = true;
//     const isGasEstimateWithDummyProof = false;
//     return gasEstimateResponse(
//       200n,
//       undefined, // broadcasterFeeCommitment
//       isGasEstimateWithDummyProof
//     );
//   } catch (err) {
//     console.log("❌ gasEstimateForEncryptBaseToken error:", err);
//   }
// };
const txidVersion = getTestTXIDVersion();
(async () => {
  console.log("🔧 Initializing DOP Engine...");
  try {
    await initEngine({
      engineName: "DOP Engine",
      dbPath: "database/DOP.db",
      shouldDebug: false,
      useNativeArtifacts: true,
      skipMerkletreeScans: false,
    });
    await initTestEngineNetworks();
    console.log("🔧 DOP Engine initialized successfully.");
    const dopWalletInfo = await createDopWallet(
      MOCK_DB_ENCRYPTION_KEY,
      MOCK_MNEMONIC,
      undefined // creationBlockNumbers
    );
    console.log("DOP Wallet Info:", dopWalletInfo);

    const chain = { type: 0, id: 1 };

    try {
      const encryptPrivateKey = ByteUtils.randomHex(32);
      console.log(txidVersion);
      const rsp = await gasEstimateForEncryptBaseToken(
        txidVersion,
        NetworkName.Polygon,
        dopWalletInfo.dopAddress,
        encryptPrivateKey,
        MOCK_TOKEN_AMOUNTS[0],
        MOCK_ETH_WALLET_ADDRESS
      );
      console.log("Gas estimate response:", rsp);
    } catch (scanErr) {
      console.error("❌ Scan failed:", scanErr);
    }
  } catch (err) {
    console.error("❌ Engine initialization error:", err);
  } finally {
    await closeEngine();
    console.log("🛑 Engine closed.");
  }
})();
