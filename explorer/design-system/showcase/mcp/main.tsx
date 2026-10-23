import { render, type ComponentType } from "preact";
import "../../styles.css";
import { documentClasses } from "@ds";
import { ShowcaseRegion } from "@ds/review";

export function mountScenario(Scenario: ComponentType, container: HTMLElement) {
  const classes = documentClasses().split(" ").filter(name => !document.body.classList.contains(name));
  document.body.classList.add(...classes);
  try {
    render(<ShowcaseRegion regions={["app"]}><ShowcaseRegion regions={["body"]}><Scenario /></ShowcaseRegion></ShowcaseRegion>, container);
  } catch (error) {
    document.body.classList.remove(...classes);
    throw error;
  }
  return () => {
    render(null, container);
    document.body.classList.remove(...classes);
  };
}
