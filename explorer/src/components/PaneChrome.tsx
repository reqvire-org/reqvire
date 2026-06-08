import logoMarkUrl from "../assets/logo-mark.svg?url";

export function ReqvireRailMark({ className = "" }: { className?: string }) {
  return (
    <img
      src={logoMarkUrl}
      className={["ex-rail-mark", className].filter(Boolean).join(" ")}
      alt=""
      aria-hidden="true"
    />
  );
}

export function PaneChromeHeader({ title }: { title: string }) {
  return (
    <div className="ex-pane-chrome-header">
      <ReqvireRailMark className="ex-header-mark" />
      <span>{title}</span>
    </div>
  );
}
