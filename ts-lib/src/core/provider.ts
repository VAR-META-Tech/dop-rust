// core/provider.ts

import { FallbackProviderJsonConfig, NetworkName } from "dop-sharedmodels-v3";
import { loadProvider } from "dop-wallet-v3";

export const loadNetworkProvider = async (
  config: FallbackProviderJsonConfig,
  network: NetworkName,
  pollingInterval: number = 10000
) => {
  return await loadProvider(config, network, pollingInterval);
};
