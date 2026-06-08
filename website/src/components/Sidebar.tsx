import { useState } from "react";
import { Link, useLocation } from "react-router-dom";
import {
  Menu,
  X,
  ExternalLink,
} from "lucide-react";
import { ReqvireLogo } from "@/components/ReqvireLogo";

const navigation = [
  { name: "About Reqvire", href: "/" },
  {
    group: "Documentation",
    items: [
      { name: "Strategic Vision", href: "/strategic-vision" },
      { name: "Semantic Model", href: "/semantic-model" },
      {
        name: "Requirements & Capabilities",
        href: "/requirements-capabilities",
      },
      { name: "Ontologies", href: "/ontologies" },
      { name: "Modeling Language", href: "/modeling-language" },
      { name: "User Guide", href: "/user-guide" },
    ],
  },
  {
    group: "Advanced",
    items: [
      { name: "Advanced", href: "/advanced" },
      { name: "Verifications", href: "/verifications" },
      { name: "Implementation Coverage", href: "/implementation-coverage" },
      { name: "Submodels & Subgraphs", href: "/submodels" },
    ],
  },
  {
    group: "Integrations",
    items: [
      { name: "Integrations", href: "/integrations" },
      { name: "MCP Server", href: "/mcp-server" },
      { name: "Coding Assistants", href: "/coding-assistants" },
    ],
  },
];

export function Sidebar() {
  const location = useLocation();
  const [mobileOpen, setMobileOpen] = useState(false);

  const isActive = (href: string) => {
    if (href === "/") return location.pathname === "/";
    return location.pathname.startsWith(href);
  };

  const sidebarContent = (
    <div className="flex flex-col h-full">
      {/* Logo */}
      <div className="px-5 py-5 border-b border-white/[0.06]">
        <Link to="/" className="inline-block text-white" onClick={() => setMobileOpen(false)}>
          <ReqvireLogo className="h-10 w-[122px]" aria-label="Reqvire" />
        </Link>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-4 px-3 space-y-5">
        <Link
          to="/"
          className={`flex items-center gap-2.5 px-3 py-2 rounded-md text-sm transition-all duration-150 ${
            isActive("/")
              ? "bg-white/[0.06] text-white border-l-[3px] border-blue-500"
              : "text-zinc-400 hover:bg-white/[0.04] hover:text-zinc-200 border-l-[3px] border-transparent"
          }`}
          onClick={() => setMobileOpen(false)}
        >
          About Reqvire
        </Link>

        {navigation.slice(1).map((group) => (
          <div key={group.group}>
            <div className="px-3 mb-1.5 text-[10px] font-semibold uppercase tracking-wider text-zinc-600">
              {group.group}
            </div>
            <div className="space-y-0.5">
              {group.items?.map((item) => (
                <Link
                  key={item.href}
                  to={item.href}
                  className={`flex items-center gap-2.5 px-3 py-2 rounded-md text-sm transition-all duration-150 ${
                    isActive(item.href)
                      ? "bg-white/[0.06] text-white border-l-[3px] border-blue-500"
                      : "text-zinc-400 hover:bg-white/[0.04] hover:text-zinc-200 border-l-[3px] border-transparent"
                  }`}
                  onClick={() => setMobileOpen(false)}
                >
                  {item.name}
                </Link>
              ))}
            </div>
          </div>
        ))}
      </nav>

      {/* Bottom */}
      <div className="border-t border-white/[0.06] px-3 py-4">
        <a
          href="https://github.com/reqvire-org/reqvire"
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center gap-2.5 px-3 py-2 text-sm text-zinc-400 hover:text-zinc-200 hover:bg-white/[0.04] rounded-md transition-colors"
        >
          <ExternalLink className="w-4 h-4" />
          Reqvire on GitHub
        </a>
      </div>
    </div>
  );

  return (
    <>
      <button
        onClick={() => setMobileOpen(!mobileOpen)}
        className="lg:hidden fixed top-4 left-4 z-50 p-2 rounded-md bg-[#0F0F0F] text-white border border-white/10"
      >
        {mobileOpen ? <X className="w-5 h-5" /> : <Menu className="w-5 h-5" />}
      </button>

      {mobileOpen && (
        <div className="lg:hidden fixed inset-0 z-40 bg-black/50" onClick={() => setMobileOpen(false)} />
      )}

      <aside
        className={`fixed lg:sticky top-0 left-0 z-40 w-[280px] h-screen bg-[#0F0F0F] flex-shrink-0 transition-transform duration-300 ${
          mobileOpen ? "translate-x-0" : "-translate-x-full lg:translate-x-0"
        }`}
      >
        {sidebarContent}
      </aside>
    </>
  );
}
