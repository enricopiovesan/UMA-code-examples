export function evaluate(evt: { id: string; tags: string[] }) {
  // simple rule based scoring
  const score = evt.tags.includes("even") ? 0.7 : 0.3;
  return { id: evt.id, score };
}
