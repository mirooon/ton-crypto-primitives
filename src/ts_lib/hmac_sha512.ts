#!/usr/bin/env tsx
import { hmac_sha512 } from '@ton/crypto-primitives';
import * as process from 'process';

async function main() {
    const key = process.argv[2] || '';
    const message = process.argv[3] || '';

    const keyBuffer = Buffer.from(key, 'utf8');
    const messageBuffer = Buffer.from(message, 'utf8');

    // Compute HMAC-SHA512
    const hmacBuffer = await hmac_sha512(keyBuffer, messageBuffer);

    // Output the result in hexadecimal format
    console.log(hmacBuffer.toString('hex'));
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
