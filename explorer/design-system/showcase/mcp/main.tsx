import { render, type ComponentType } from "preact";
import "../../styles.css";
import { documentClasses } from "@ds";
import { ShowcaseRegion } from "@ds/review";

export function mountScenario(Scenario: ComponentType, container: HTMLElement) {
  const classes = documentClasses().split(" ").filter(name => !document.body.classList.contains(name));
  document.body.classList.add(...classes);
  try {
    // The preview body scrolls, so it is a keyboard-focusable, labelled region.
    render(<ShowcaseRegion regions={["app"]}><ShowcaseRegion regions={["body"]} as="section" aria-label="Scenario preview" tabIndex={0}><Scenario /></ShowcaseRegion></ShowcaseRegion>, container);
  } catch (error) {
    document.body.classList.remove(...classes);
    throw error;
  }
  return () => {
    render(null, container);
    document.body.classList.remove(...classes);
  };
}
