import { pbkdf2, getRandomBytes } from 'dop-wallet-v3';

export const getRandomBytesCall = (length: number) => {
    const bytes = getRandomBytes(length);
    return bytes;
}

export const hashPasswordString = async ( secret: string, salt: string, iterations: number ): Promise<string> => {
    return pbkdf2(secret, salt, iterations);
}


export const pbkdf2Lib = async (): Promise<string> => {
    return pbkdf2('password', 'salt', 100000);
}

