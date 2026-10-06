import { css } from "@linaria/core";
import { cx } from "@linaria/atomic";
import { useEffect, useId, useRef, useState, type KeyboardEvent } from "react";
import { Icon } from "../../components/core/Icon";

const selectorClass = css`
  position: relative;
  flex-shrink: 0;
  width: 100%;
  min-width: 0;
  color: var(--text-body);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
`;
const triggerClass = css`
  display: flex;
  align-items: center;
  gap: var(--space-5);
  width: 100%;
  min-width: 0;
  height: var(--control-lg);
  padding: 0 var(--space-6);
  border: var(--border-w) solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--bg-surface);
  color: var(--text-strong);
  font: inherit;
  font-weight: var(--weight-medium);
  text-align: left;
  cursor: pointer;
  transition: background var(--dur-fast), border-color var(--dur-fast);
  &:hover, &[aria-expanded="true"] { background: var(--bg-sunken); border-color: var(--border-strong); }
  &:focus-visible { outline: none; box-shadow: var(--ring-focus); }
  svg { color: var(--text-muted); width: var(--icon-sm); height: var(--icon-sm); }
`;
const nameClass = css`
  flex: 1 1 auto;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
`;
const compactTriggerClass = css`
  height: var(--control-sm);
  gap: var(--space-4);
  padding-inline: var(--space-4);
  border-color: transparent;
  background: transparent;
`;
const menuClass = css`
  position: absolute;
  z-index: var(--z-popover);
  top: calc(100% + var(--space-2));
  left: 0;
  right: 0;
  padding: var(--space-3);
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-overlay);
  box-shadow: var(--shadow-lg);
`;
const headingClass = css`
  padding: var(--space-4) var(--space-5);
  color: var(--text-muted);
  font-size: var(--text-caption);
  font-weight: var(--weight-medium);
`;
const compactMenuClass = css`
  right: auto;
  width: max(100%, calc(var(--space-32) * 5));
  max-width: calc(100vw - var(--space-32));
  box-sizing: border-box;
`;
const listClass = css`
  max-height: min(50vh, calc(var(--control-lg) * 7));
  overflow-y: auto;
  overscroll-behavior: contain;
  margin: 0;
  padding: 0;
  list-style: none;
`;
const optionClass = css`
  display: flex;
  align-items: center;
  gap: var(--space-5);
  padding: var(--space-5);
  border-radius: var(--radius-sm);
  cursor: pointer;
  &[data-active="true"] { background: var(--bg-sunken); }
  &[aria-selected="true"] { background: var(--accent-subtle); }
  &[aria-selected="true"] > svg { color: var(--accent); }
  &[aria-disabled="true"] { color: var(--text-muted); cursor: default; }
  svg { flex: 0 0 auto; width: var(--icon-sm); height: var(--icon-sm); }
`;
const optionTextClass = css`
  flex: 1 1 auto;
  min-width: 0;
  line-height: var(--leading-normal);
  overflow-wrap: anywhere;
  strong { display: block; font-weight: var(--weight-medium); }
  small { display: block; color: var(--text-muted); font-size: var(--text-caption); }
`;

export interface WorktreeSelectorProps {
  density?: "default" | "compact";
  value?: string;
  displayedValue?: string;
  choices: { id: string; branch: string; root: string; available: boolean }[];
  branch?: string | null;
  pending?: boolean;
  onChange: (id: string) => void;
  onOpen: () => void;
}
/** The trigger identifies the adopted snapshot; choosing a target does not relabel it. */
export function WorktreeSelector({ density = "default", value, displayedValue, choices, branch, pending, onChange, onOpen }: WorktreeSelectorProps) {
  const id = useId();
  const root = useRef<HTMLDivElement>(null);
  const trigger = useRef<HTMLButtonElement>(null);
  const [open, setOpen] = useState(false);
  const [activeId, setActiveId] = useState<string>();
  const typed = useRef({ text: "", at: 0 });
  const currentId = displayedValue ?? value;
  const current = choices.find(choice => choice.id === currentId);
  const available = choices.filter(choice => choice.available);
  const active = available.find(choice => choice.id === activeId) ?? available.find(choice => choice.id === currentId) ?? available[0];
  const optionId = (choiceId: string) => `${id}-${choices.findIndex(choice => choice.id === choiceId)}`;

  useEffect(() => {
    if (!open) return;
    const dismiss = (event: PointerEvent) => {
      if (event.target instanceof Node && !root.current?.contains(event.target)) setOpen(false);
    };
    document.addEventListener("pointerdown", dismiss);
    return () => document.removeEventListener("pointerdown", dismiss);
  }, [open]);
  useEffect(() => {
    if (open && active) document.getElementById(optionId(active.id))?.scrollIntoView?.({ block: "nearest" });
  }, [open, active?.id, id, choices]);

  function reveal() {
    setActiveId(available.find(choice => choice.id === currentId)?.id ?? available[0]?.id);
    setOpen(true);
    onOpen();
  }
  function choose(choice: WorktreeSelectorProps["choices"][number]) {
    if (!choice.available) return;
    setOpen(false);
    trigger.current?.focus();
    onChange(choice.id);
  }
  function keyDown(event: KeyboardEvent<HTMLButtonElement>) {
    if (event.key === "Escape" || event.key === "Tab") {
      if (open && event.key === "Escape") { event.preventDefault(); event.stopPropagation(); }
      setOpen(false);
    } else if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      if (open) { if (active) choose(active); }
      else reveal();
    } else if (["ArrowDown", "ArrowUp", "Home", "End"].includes(event.key)) {
      event.preventDefault();
      if (!open) reveal();
      const index = available.findIndex(choice => choice.id === active?.id);
      const next = event.key === "Home" ? 0 : event.key === "End" ? available.length - 1
        : !open ? Math.max(index, 0) : (index + (event.key === "ArrowDown" ? 1 : -1) + available.length) % available.length;
      setActiveId(available[next]?.id);
    } else if (event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
      event.preventDefault();
      if (!open) reveal();
      const now = Date.now();
      typed.current = { text: (now - typed.current.at < 700 ? typed.current.text : "") + event.key.toLowerCase(), at: now };
      const match = available.find(choice => choice.branch.toLowerCase().startsWith(typed.current.text));
      if (match) setActiveId(match.id);
    }
  }

  return <div ref={root} data-density={density} className={cx("ux-worktree-selector", selectorClass)} onBlur={event => {
    if (!event.currentTarget.contains(event.relatedTarget)) setOpen(false);
  }}>
    <button ref={trigger} type="button" role="combobox" aria-label="Branch" aria-haspopup="listbox"
      aria-expanded={open} aria-controls={open ? id : undefined} aria-activedescendant={open && active ? optionId(active.id) : undefined}
      aria-busy={pending || undefined} title={[branch, current?.root, pending ? "Loading selected branch…" : "Switch branch"].filter(Boolean).join("\n")}
      className={cx(triggerClass, density === "compact" && compactTriggerClass)} onClick={() => open ? setOpen(false) : reveal()} onKeyDown={keyDown}>
      <Icon name={pending ? "rotate-ccw" : "git-branch"} />
      <span className={cx(nameClass)}>{branch || "Choose branch"}</span>
      <Icon name="chevron-down" />
    </button>
    {open && <div className={cx(menuClass, density === "compact" && compactMenuClass)}>
      <div className={cx(headingClass)}>Switch branch</div>
      <ul id={id} role="listbox" aria-label="Branches" className={cx(listClass)}>
        {choices.map(choice => <li key={choice.id} id={optionId(choice.id)} role="option" data-worktree-id={choice.id}
          aria-selected={choice.id === currentId} aria-disabled={!choice.available || undefined}
          data-active={choice.id === active?.id} className={cx(optionClass)}
          onPointerMove={() => { if (choice.available) setActiveId(choice.id); }}
          onPointerDown={event => event.preventDefault()} onClick={() => choose(choice)}>
          <Icon name="git-branch" />
          <span className={cx(optionTextClass)}><strong>{choice.branch}</strong><small>{choice.root}</small>
            {!choice.available && <small>Unavailable</small>}
          </span>
          {choice.id === currentId && <Icon name="check" />}
        </li>)}
      </ul>
      {(!choices.length || pending) && <div role="status" className={cx(headingClass)}>
        {pending ? `Loading ${choices.find(choice => choice.id === value)?.branch ?? "selected branch"}…` : "No branches available"}
      </div>}
    </div>}
  </div>;
}
