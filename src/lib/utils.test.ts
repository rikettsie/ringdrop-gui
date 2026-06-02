import { describe, it, expect } from 'vitest';
import { truncateHash, formatBytes } from './utils';

describe('truncateHash', () => {
  it('truncates a long hash and appends ellipsis', () => {
    expect(truncateHash('abcdef1234567890', 8)).toBe('abcdef12…');
  });

  it('leaves a hash shorter than the limit untouched', () => {
    expect(truncateHash('abc', 8)).toBe('abc');
  });

  it('leaves a hash of exactly the limit length untouched', () => {
    expect(truncateHash('abcdefgh', 8)).toBe('abcdefgh');
  });
});

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
