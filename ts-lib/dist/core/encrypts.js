import { pbkdf2, getRandomBytes } from 'dop-wallet-v3';
export const getRandomBytesCall = (length) => {
    const bytes = getRandomBytes(length);
    return bytes;
};
export const hashPasswordString = async (secret, salt, iterations) => {
    return pbkdf2(secret, salt, iterations);
};
export const pbkdf2Lib = async () => {
    return pbkdf2('password', 'salt', 100000);
};
