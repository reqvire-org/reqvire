import { createContext, useContext, useMemo, type ReactNode } from "react";
import type {
  ExplorerProjectStore,
  ProjectStoreElement,
} from "./types";

export interface StoreContextValue {
  store: ExplorerProjectStore;
  /** Non-null when the seed schema version differs from the expected one. */
  schemaMismatch: string | null;
  /** O(1) element lookup by full identifier. */
  elementById: (id: string) => ProjectStoreElement | undefined;
}

const StoreContext = createContext<StoreContextValue | null>(null);

export function StoreProvider({
  store,
  schemaMismatch,
  children,
}: {
  store: ExplorerProjectStore;
  schemaMismatch: string | null;
  children: ReactNode;
}) {
  const value = useMemo<StoreContextValue>(() => {
    const index = new Map<string, ProjectStoreElement>();
    for (const element of store.elements) {
      index.set(element.id, element);
    }
    return {
      store,
      schemaMismatch,
      elementById: (id: string) => index.get(id),
    };
  }, [store, schemaMismatch]);

  return <StoreContext.Provider value={value}>{children}</StoreContext.Provider>;
}

export function useStore(): StoreContextValue {
  const ctx = useContext(StoreContext);
  if (!ctx) {
    throw new Error("useStore must be used within a StoreProvider");
  }
  return ctx;
}
