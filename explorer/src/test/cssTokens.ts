/** Resolve simple aliases from the real foundation CSS where jsdom leaves var() unevaluated. */
export function readCssToken(name: string): string {
  const style = getComputedStyle(document.documentElement);
  const seen = new Set<string>();
  let current = name;
  while (!seen.has(current)) {
    seen.add(current);
    const value = style.getPropertyValue(current).trim();
    const alias = /^var\((--[\w-]+)\)$/.exec(value);
    if (!alias) return value;
    current = alias[1];
  }
  throw new Error(`Cyclic CSS token alias: ${name}`);
}
