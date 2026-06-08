import { Outlet } from "react-router-dom";
import { Sidebar } from "./Sidebar";

export function AppLayout() {
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
