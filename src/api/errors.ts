export function describeError(e: unknown): string {
  if (typeof e === "object" && e !== null && "message" in e) {
    return String((e as { message: unknown }).message);
  }
  return String(e);
}

export function errorCode(e: unknown): string | null {
  if (typeof e === "object" && e !== null && "code" in e) {
    return String((e as { code: unknown }).code);
  }
  return null;
}