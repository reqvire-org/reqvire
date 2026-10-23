// ADS exports this development helper as JavaScript without declarations.
// Keep its browser boundary typed without enabling untyped JS across Explorer.
declare module "@reqvire-org/ads/verification" {
  export function openBehaviorScenario(
    page: import("@playwright/test").Page,
    checkId: string,
    options?: { timeout?: number },
  ): Promise<import("@playwright/test").Frame>;
}
