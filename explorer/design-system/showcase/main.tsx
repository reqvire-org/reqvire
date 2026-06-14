import React from "react";
import { createRoot } from "react-dom/client";
import "../styles.css";
import "./showcase.css";
import { ShowcaseApp } from "./ShowcaseApp";

const container = document.getElementById("root");
if (!container) throw new Error("Missing root element #root");

createRoot(container).render(
  <React.StrictMode>
    <ShowcaseApp />
  </React.StrictMode>,
);
