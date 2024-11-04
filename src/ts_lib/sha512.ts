#!/usr/bin/env tsx
import { sha512 } from '@ton/crypto-primitives';
import * as process from 'process';

async function main() {
    const input = process.argv[2] || '';
    const inputBuffer = Buffer.from(input, 'utf8');

    // Compute SHA512 hash
    const hashBuffer = await sha512(inputBuffer);

    // Output the result in hexadecimal format
    console.log(hashBuffer.toString('hex'));
}

main().catch(err => {
    console.error(err);
    process.exit(1);
});
