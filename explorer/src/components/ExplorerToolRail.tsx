import type { ReactNode } from "react";
import { IconButton, Tooltip } from "@radix-ui/themes";
import {
  ActivityLogIcon,
  CubeIcon,
  GearIcon,
  GlobeIcon,
  InfoCircledIcon,
  LayersIcon,
  MagnifyingGlassIcon,
  QuestionMarkCircledIcon,
  Share2Icon,
} from "@radix-ui/react-icons";
import type { ViewId } from "../router/routes";

interface ExplorerToolRailProps {
  onNavigate: (view: ViewId) => void;
  onOpenHelp: () => void;
}

export function ExplorerToolRail({ onNavigate, onOpenHelp }: ExplorerToolRailProps) {
  return (
    <aside className="explorer-tool-rail" aria-label="Explorer tools">
      <ToolButton label="Inspector">
        <InfoCircledIcon />
      </ToolButton>
      <ToolButton label="Search" onClick={() => onNavigate("search")}>
        <MagnifyingGlassIcon />
      </ToolButton>
      <ToolButton label="Model" onClick={() => onNavigate("model")}>
        <CubeIcon />
      </ToolButton>
      <ToolButton label="Knowledge Graph" onClick={() => onNavigate("knowledge-graph")}>
        <Share2Icon />
      </ToolButton>
      <ToolButton label="Ontologies" onClick={() => onNavigate("ontologies")}>
        <GlobeIcon />
      </ToolButton>
      <ToolButton label="Traces" onClick={() => onNavigate("traces")}>
        <ActivityLogIcon />
      </ToolButton>
      <ToolButton label="KN2" onClick={() => onNavigate("kn2")}>
        <LayersIcon />
      </ToolButton>
      <ToolButton label="Settings">
        <GearIcon />
      </ToolButton>
      <div className="explorer-tool-rail-spacer" />
      <ToolButton label="Help" onClick={onOpenHelp}>
        <QuestionMarkCircledIcon />
      </ToolButton>
    </aside>
  );
}

function ToolButton({
  label,
  onClick,
  children,
}: {
  label: string;
  onClick?: () => void;
  children: ReactNode;
}) {
  return (
    <Tooltip content={label} side="left">
      <IconButton
        aria-label={label}
        variant="ghost"
        color="gray"
        className="explorer-tool-button"
        onClick={onClick}
      >
        {children}
      </IconButton>
    </Tooltip>
  );
}
