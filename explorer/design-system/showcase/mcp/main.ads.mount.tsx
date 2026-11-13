import { render, type ComponentType } from "preact";
import { createPreviewRoot } from "@reqvire-org/ads/preview";
import { documentClasses } from "@ds";

// ADS owns preview framing; the product document skin (typography, links and
// scrollbars) is applied to the body as in the application, including portals.
export function mountScenario(Scenario: ComponentType, container: HTMLElement) {
  const classes = documentClasses().split(" ").filter(name => !document.body.classList.contains(name));
  document.body.classList.add(...classes);
  const frame = createPreviewRoot(container);
  try {
    render(<Scenario />, frame);
  } catch (error) {
    frame.remove();
    document.body.classList.remove(...classes);
    throw error;
  }
  return () => {
    render(null, frame);
    frame.remove();
    document.body.classList.remove(...classes);
  };
}
