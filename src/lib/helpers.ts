export function areArraysEqual<T>(arr1: T[], arr2: T[]): boolean {
  if (arr1 === arr2) return true;

  if (arr1.length !== arr2.length) return false;

  for (let i = 0; i < arr1.length; i++) {
    if (arr1[i] !== arr2[i]) return false;
  }

  return true;
}

export function formatDuration(seconds: bigint | number): string {
  const s = Number(seconds);
  const m = Math.floor(s / 60);
  const rem = Math.floor(s % 60);
  return `${m}:${rem.toString().padStart(2, '0')}`;
}