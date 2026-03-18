import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { evalFlag, rollout } from './lib.mjs';

const root = join(import.meta.dirname, '..', '..');

function loadLab(name) {
  return JSON.parse(readFileSync(join(root, 'labs', 'inputs', `${name}.json`), 'utf8'));
}

test('country rule wins before rollout', () => {
  const result = evalFlag(loadLab('lab1-country-match'));
  assert.deepEqual(result, { key: 'paywall', enabled: true, matchedRule: 0 });
});

test('rollout is deterministic for the sticky cohort', () => {
  assert.equal(rollout('paywall', 'u20', 0.2), true);
  const result = evalFlag(loadLab('lab2-rollout-match'));
  assert.deepEqual(result, { key: 'paywall', enabled: true, matchedRule: 1 });
});

test('default fallback returns null matchedRule', () => {
  const result = evalFlag(loadLab('lab3-default-fallback'));
  assert.deepEqual(result, { key: 'paywall', enabled: false, matchedRule: null });
});

test('rule language lab exercises in numeric and logical operators', () => {
  const result = evalFlag(loadLab('lab4-rule-language'));
  assert.deepEqual(result, { key: 'checkout-rollout', enabled: true, matchedRule: 0 });
});
