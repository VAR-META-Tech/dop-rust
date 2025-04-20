// core/logger.ts
import { setLoggers } from "dop-wallet-v3";
export const setEngineLoggers = (log = console.log, error = console.error) => {
    setLoggers(log, error);
};
