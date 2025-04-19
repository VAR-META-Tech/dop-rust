// core/provider.ts
import { loadProvider } from 'dop-wallet-stagging';
export const loadNetworkProvider = async (config, network, pollingInterval = 10000) => {
    return await loadProvider(config, network, pollingInterval);
};
