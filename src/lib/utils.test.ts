import { describe, it, expect } from 'vitest';
import { formatBytes, formatRemaining } from './utils';

describe('formatBytes', () => {
  it('formats zero bytes', () => {
    expect(formatBytes(0)).toBe('0 B');
  });

  it('formats kilobytes', () => {
    expect(formatBytes(1024)).toBe('1.0 KB');
  });

  it('formats megabytes', () => {
    expect(formatBytes(1.5 * 1024 * 1024)).toBe('1.5 MB');
  });
});

describe('formatRemaining', () => {
  const nowMs = 1_800_000_000_000;
  const nowSecs = nowMs / 1000;

  it.each([
    [604_799, '6d 23h'],
    [2_710, '45m 10s'],
    [86_430, '1d'],
    [30, '30s'],
  ])('formats %i seconds left as %s', (left, expected) => {
    expect(formatRemaining(nowSecs + left, nowMs)).toBe(expected);
  });

  it('clamps an already-passed expiry to 0s', () => {
    expect(formatRemaining(nowSecs - 60, nowMs)).toBe('0s');
  });
});
