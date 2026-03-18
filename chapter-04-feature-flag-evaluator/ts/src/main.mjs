import { stdin } from 'node:process';
import { evalFlag } from './lib.mjs';

async function readStdin() {
  const chunks = [];
  for await (const chunk of stdin) {
    chunks.push(chunk);
  }
  return Buffer.concat(chunks).toString('utf8');
}

const input = JSON.parse(await readStdin());
process.stdout.write(JSON.stringify(evalFlag(input)));
