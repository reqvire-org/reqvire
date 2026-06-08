import type { ReactNode } from "react";

export function Section({
  title,
  children,
}: {
  title: string;
  children: ReactNode;
}) {
  return (
    <section className="mb-12">
      <h2 className="text-2xl font-semibold text-zinc-900 mb-4">{title}</h2>
      {children}
    </section>
  );
}

export function CodeBlock({ children }: { children: string }) {
  return (
    <div className="bg-zinc-900 rounded-lg p-5 overflow-x-auto">
      <pre className="text-sm text-zinc-300 font-mono whitespace-pre">{children}</pre>
    </div>
  );
}

export function BulletList({ items }: { items: string[] }) {
  return (
    <ul className="space-y-2.5">
      {items.map((item) => (
        <li key={item} className="flex items-start gap-2.5">
          <span className="w-1.5 h-1.5 rounded-full bg-blue-500 mt-2 flex-shrink-0" />
          <span className="text-zinc-700">{item}</span>
        </li>
      ))}
    </ul>
  );
}

export function TermList({ items }: { items: [string, string][] }) {
  return (
    <div className="space-y-3">
      {items.map(([term, desc]) => (
        <div key={term} className="flex items-start gap-3">
          <span className="w-1.5 h-1.5 rounded-full bg-blue-500 mt-2 flex-shrink-0" />
          <p className="text-zinc-700">
            <strong className="text-zinc-900">{term}</strong> - {desc}
          </p>
        </div>
      ))}
    </div>
  );
}

export function CommandList({
  items,
}: {
  items: { cmd: string; desc: string }[];
}) {
  return (
    <div className="space-y-3">
      {items.map(({ cmd, desc }) => (
        <div
          key={cmd}
          className="flex flex-col sm:flex-row sm:items-start gap-2"
        >
          <code className="text-sm font-mono text-zinc-800 bg-zinc-100 px-2 py-1 rounded sm:min-w-[300px] flex-shrink-0">
            {cmd}
          </code>
          <span className="text-sm text-zinc-600">{desc}</span>
        </div>
      ))}
    </div>
  );
}

export function DetailGrid({
  items,
}: {
  items: { name: string; desc: string }[];
}) {
  return (
    <div className="space-y-4">
      {items.map((item) => (
        <div key={item.name} className="border border-zinc-200 rounded-lg p-4">
          <h4 className="font-semibold text-zinc-900 mb-1">{item.name}</h4>
          <p className="text-sm text-zinc-600 leading-relaxed">{item.desc}</p>
        </div>
      ))}
    </div>
  );
}
