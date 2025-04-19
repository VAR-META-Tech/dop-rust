// core/logger.ts
import { setLoggers as dopSetLoggers } from 'dop-wallet-stagging';

export const setEngineLoggers = (
  log: (msg: string) => void = console.log,
  error: (msg: string) => void = console.error
) => {
  dopSetLoggers(log, error);
};
