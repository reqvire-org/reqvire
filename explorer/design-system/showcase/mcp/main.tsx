import { render, type ComponentType } from "preact";
import "../../styles.css";
import { ShowcaseRegion } from "@ds";

export function mountScenario(Scenario: ComponentType, container: HTMLElement) {
  render(<ShowcaseRegion regions={["app"]}><ShowcaseRegion regions={["body"]}><Scenario /></ShowcaseRegion></ShowcaseRegion>, container);
  return () => render(null, container);
}
