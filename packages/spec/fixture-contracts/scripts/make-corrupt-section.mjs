#!/usr/bin/env node
/**
 * Truncate a WASM module's `contractspecv0` custom section mid-entry.
 *
 * Reads a contract WASM (default: the committed identity-registry fixture),
 * cuts the spec section payload in half, and writes the result. The module
 * header and section framing stay valid, so the corruption is isolated to the
 * spec entries themselves: a reader must report an unreadable interface,
 * not a malformed module.
 *
 * Deterministic: the same input bytes always produce the same output bytes,
 * so CI rebuilds this from source and compares it like every other fixture.
 *
 * Usage:
 *   node scripts/make-corrupt-section.mjs [input.wasm] [output.wasm]
 */
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { Buffer } from 'node:buffer';

const __dirname = dirname(fileURLToPath(import.meta.url));
const args = process.argv.slice(2);
const inputPath = args[0] ?? join(__dirname, '..', '..', 'fixtures', 'identity-registry.wasm');
const outputPath = args[1] ?? join(__dirname, '..', '..', 'fixtures', 'corrupt_section.wasm');

function readLebU(bytes, pos) {
  let result = 0;
  let shift = 0;
  for (;;) {
    const byte = bytes[pos++];
    result |= (byte & 0x7f) << shift;
    shift += 7;
    if ((byte & 0x80) === 0) break;
  }
  return [result >>> 0, pos];
}

function writeLebU(value) {
  const out = [];
  do {
    let byte = value & 0x7f;
    value >>>= 7;
    if (value !== 0) byte |= 0x80;
    out.push(byte);
  } while (value !== 0);
  return Buffer.from(out);
}

const wasm = readFileSync(inputPath);
if (wasm.subarray(0, 4).toString('binary') !== '\0asm' || wasm.readUInt32LE(4) !== 1) {
  throw new Error(`${inputPath} is not a WASM module`);
}

let pos = 8;
let specStart = -1;
let specEnd = -1;
let specNameLen = 0;
while (pos < wasm.length) {
  const sectionStart = pos;
  const id = wasm[pos++];
  let size;
  [size, pos] = readLebU(wasm, pos);
  const payloadStart = pos;
  const payloadEnd = pos + size;
  if (id === 0) {
    let nameLen;
    [nameLen, pos] = readLebU(wasm, pos);
    const name = wasm.subarray(pos, pos + nameLen).toString('utf8');
    if (name === 'contractspecv0') {
      specStart = sectionStart;
      specEnd = payloadEnd;
      specNameLen = pos + nameLen - payloadStart;
    }
    pos = payloadEnd;
  } else {
    pos = payloadEnd;
  }
}

if (specStart === -1) throw new Error(`${inputPath} has no contractspecv0 section`);

const prefix = wasm.subarray(0, specStart);
// Re-parse the section to split framing from payload.
let p = specStart + 1; // section id
let size;
[size, p] = readLebU(wasm, p);
const payloadStart = p;
let nameLen;
[nameLen, p] = readLebU(wasm, p);
const headerLen = p + nameLen - payloadStart; // name length prefix + name
const data = wasm.subarray(payloadStart + headerLen, specEnd);
// Cut the entry data in half: guaranteed mid-entry for a multi-entry spec,
// while the section framing stays well-formed.
const kept = data.subarray(0, Math.floor(data.length / 2));
const nameBytes = wasm.subarray(payloadStart, payloadStart + headerLen);
const newPayload = Buffer.concat([nameBytes, kept]);
const newSection = Buffer.concat([Buffer.from([0]), writeLebU(newPayload.length), newPayload]);
const out = Buffer.concat([prefix, newSection, wasm.subarray(specEnd)]);

writeFileSync(outputPath, out);
console.log(
  `corrupt_section: ${inputPath} spec data ${data.length} -> ${kept.length} bytes, module ${wasm.length} -> ${out.length} bytes`,
);
