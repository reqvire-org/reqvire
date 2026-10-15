import { render } from "preact";
import "../styles.css";
import { ShowcaseApp } from "./ShowcaseApp";

const container = document.getElementById("root");
if (!container) throw new Error("Missing root element #root");

render(<ShowcaseApp />, container);
