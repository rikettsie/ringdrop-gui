/** Formats a byte count as a human-readable string (e.g. "1.2 MB"). */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}

/**
 * Formats the time left until `expiresAtSecs` (Unix seconds) using its two
 * most significant adjacent units, e.g. "6d 23h" or "45m 10s".
 */
export function formatRemaining(expiresAtSecs: number, nowMs: number = Date.now()): string {
  const units: [number, string][] = [[86_400, 'd'], [3_600, 'h'], [60, 'm'], [1, 's']];
  const secs = Math.max(0, Math.floor(expiresAtSecs - nowMs / 1000));
  const major = units.findIndex(([unitSecs]) => secs >= unitSecs);
  if (major === -1) return '0s';
  const [majorSecs, majorSuffix] = units[major];
  const majorPart = `${Math.floor(secs / majorSecs)}${majorSuffix}`;
  const minor = units[major + 1];
  if (!minor) return majorPart;
  const minorValue = Math.floor((secs % majorSecs) / minor[0]);
  return minorValue > 0 ? `${majorPart} ${minorValue}${minor[1]}` : majorPart;
}

/** Duration units offered when adding a peer to a ring, in seconds. */
export const EXPIRY_UNITS = [
  { label: 'hours', secs: 3_600 },
  { label: 'days', secs: 86_400 },
  { label: 'weeks', secs: 604_800 },
] as const;
