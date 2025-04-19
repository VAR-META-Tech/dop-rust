// core/logger.ts
import { setLoggers as dopSetLoggers } from 'dop-wallet-stagging';
export const setEngineLoggers = (log = console.log, error = console.error) => {
    dopSetLoggers(log, error);
};
