import { createContext } from "preact";
import { useContext } from "preact/hooks";
import type { ToastPosition } from "@ds";

interface ToastDemoSettings {
  position: ToastPosition;
  setPosition: (position: ToastPosition) => void;
  expand: boolean;
  setExpand: (expand: boolean) => void;
}

export const ToastDemoSettingsContext = createContext<ToastDemoSettings | null>(null);

export function useToastDemoSettings() {
  const context = useContext(ToastDemoSettingsContext);
  if (!context) throw new Error("Toast showcase settings must be provided at the showcase root.");
  return context;
}
