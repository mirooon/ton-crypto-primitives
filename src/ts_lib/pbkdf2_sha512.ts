#!/usr/bin/env tsx
import { pbkdf2_sha512 } from '@ton/crypto-primitives';
import * as process from 'process';

async function main() {
    const password = process.argv[2] || '';
    const salt = process.argv[3] || '';
    const iterations = parseInt(process.argv[4], 10) || 1000;
    const keyLength = parseInt(process.argv[5], 10) || 64;

    const passwordBuffer = Buffer.from(password, 'utf8');
    const saltBuffer = Buffer.from(salt, 'utf8');

    // Compute PBKDF2-SHA512
    const derivedKey = await pbkdf2_sha512(passwordBuffer, saltBuffer, iterations, keyLength);

    // Output the result in hexadecimal format
    console.log(derivedKey.toString('hex'));
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
