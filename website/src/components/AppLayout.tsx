import { useEffect } from "react";
import { Outlet, useLocation } from "react-router-dom";
import { Sidebar } from "./Sidebar";

export function AppLayout() {
  const { hash, pathname, search } = useLocation();

  useEffect(() => {
    if (typeof window === "undefined") {
      return;
    }

    if (!hash) {
      window.scrollTo({ top: 0, left: 0, behavior: "instant" });
      return;
    }

    window.requestAnimationFrame(() => {
      document.getElementById(hash.slice(1))?.scrollIntoView();
    });
  }, [hash, pathname, search]);

  return (
    <div className="flex min-h-screen bg-background">
      <Sidebar />
      <main className="flex-1 min-w-0">
        <div className="max-w-[900px] mx-auto px-6 sm:px-10 py-8 lg:py-12">
          <Outlet />
        </div>
      </main>
    </div>
  );
}
