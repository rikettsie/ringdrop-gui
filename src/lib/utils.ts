/** Truncates a hex hash to `chars` characters for display, appending an ellipsis. */
export function truncateHash(hash: string, chars = 8): string {
  return hash.length <= chars ? hash : `${hash.slice(0, chars)}…`;
}

/** Formats a byte count as a human-readable string (e.g. "1.2 MB"). */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}
