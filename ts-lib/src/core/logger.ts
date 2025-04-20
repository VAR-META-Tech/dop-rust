// core/logger.ts

import { setLoggers } from "dop-wallet-v3";

export const setEngineLoggers = (
  log: (msg: string) => void = console.log,
  error: (msg: string) => void = console.error
) => {
  setLoggers(log, error);
};
