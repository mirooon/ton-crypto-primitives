#!/usr/bin/env tsx
import { getSecureRandomBytes } from '@ton/crypto-primitives';
import * as process from 'process';

async function main() {
    const byteLength = parseInt(process.argv[2], 10) || 32;

    // Generate secure random bytes
    const randomBytes = getSecureRandomBytes(byteLength);

    // Output the result in hexadecimal format
    console.log(randomBytes.toString('hex'));
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
