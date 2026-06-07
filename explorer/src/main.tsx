import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Theme } from "@radix-ui/themes";
import { App } from "./App";
import "./styles.css";

const container = document.getElementById("root");
if (!container) {
  throw new Error("Reqvire Explorer: #root mount point not found");
}

createRoot(container).render(
  <StrictMode>
    {/* Radix Themes 3 provider. `gray`/`slate` accent keeps the MONO palette. */}
    <Theme accentColor="gray" grayColor="sand" radius="small" scaling="95%">
      <App />
    </Theme>
  </StrictMode>,
);
