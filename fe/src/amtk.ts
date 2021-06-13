import CryptoJS from 'crypto-js';

const AES_IV = CryptoJS.enc.Hex.parse('c6eb2f7f5c4740c1a2f708fefd947d39');
const PBKDF2_SALT = CryptoJS.enc.Hex.parse('9a3686ac');
const PK = '69af143c-e8cf-47f8-bf09-fc1f61e5cc33';

function kdf(pw: string): CryptoJS.lib.WordArray {
    return CryptoJS.PBKDF2(pw, PBKDF2_SALT, {
        keySize: 4,
        iterations: 1000,
    });
}

function decryptInternal(ciphertext: CryptoJS.lib.WordArray, pw: string): string {
    return CryptoJS.AES.decrypt(CryptoJS.lib.CipherParams.create({
        ciphertext,
        iv: AES_IV,
    }), kdf(pw)).toString(CryptoJS.enc.Utf8);
}

export function decrypt(encData: string) {
    const ciphertext = CryptoJS.enc.Base64.parse(encData.slice(0, encData.length-88));
    const pwEncrypted = CryptoJS.enc.Base64.parse(encData.slice(encData.length-88));

    const pwPlaintext = decryptInternal(pwEncrypted, PK).split('|')[0];
    return JSON.parse(decryptInternal(ciphertext, pwPlaintext));
}
