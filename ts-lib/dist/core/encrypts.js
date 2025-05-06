import { randomHex } from "dop-engine";
import { pbkdf2 } from 'dop-wallet-stagging';
export const getRandomBytes = (length) => {
    const bytes = randomHex(length);
    return bytes;
};
export const hashPasswordString = async (secret, salt, iterations) => {
    return pbkdf2(secret, salt, iterations);
};
export const setEncryptionKeyFromPassword = async (password) => {
    // The `password` is provided by the user
    const salt = getRandomBytes(16);
    const [encryptionKey, hashPasswordStored] = await Promise.all([
        hashPasswordString(password, salt, 100000),
        hashPasswordString(password, salt, 1000000),
    ]);
    await Promise.all([
    // Save `hashPasswordStored` in local storage ..., // Save `salt` in local storage
    ]);
    return encryptionKey;
};
export const pbkdf2Lib = async () => {
    return pbkdf2('password', 'salt', 100000);
};
export const getEncryptionKeyFromPassword = async (password) => {
    const salt = getRandomBytes(16);
    const iterations = 100000;
    const encryptionKey = await hashPasswordString(password, salt, iterations);
    return encryptionKey;
};
