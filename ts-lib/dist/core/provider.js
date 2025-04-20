// core/provider.ts
import { loadProvider } from "dop-wallet-v3";
export const loadNetworkProvider = async (config, network, pollingInterval = 10000) => {
    return await loadProvider(config, network, pollingInterval);
};
