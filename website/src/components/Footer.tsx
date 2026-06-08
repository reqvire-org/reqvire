import { Github } from "lucide-react";
import { ReqvireLogo } from "@/components/ReqvireLogo";

export function Footer() {
  return (
    <footer className="border-t border-zinc-200 mt-20 py-10 px-6">
      <div className="max-w-[768px] mx-auto flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div className="min-w-0 flex-1">
          <ReqvireLogo className="h-10 w-[122px] text-zinc-900" aria-label="Reqvire" />
          <p className="max-w-[320px] text-[13px] leading-relaxed text-zinc-500 mt-0.5">
            Build verifiable and traceable software.
          </p>
        </div>
        <div className="flex flex-shrink-0 items-center gap-4">
          <a
            href="https://github.com/reqvire-org/reqvire"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1.5 text-sm text-zinc-500 hover:text-blue-600 transition-colors"
          >
            <Github className="w-4 h-4" />
            GitHub
          </a>
          <span className="text-zinc-300">|</span>
          <span className="text-xs text-zinc-400">
            Copyright &copy; 2026 Ilija Ljubicic.
          </span>
        </div>
      </div>
    </footer>
  );
}
