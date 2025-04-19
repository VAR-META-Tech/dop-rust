// core/provider.ts
import { loadProvider } from 'dop-wallet-stagging';
import { FallbackProviderJsonConfig, NetworkName } from 'dop-sharedmodel';

export const loadNetworkProvider = async (
  config: FallbackProviderJsonConfig,
  network: NetworkName,
  pollingInterval: number = 10000
) => {
  return await loadProvider(config, network, pollingInterval);
};
