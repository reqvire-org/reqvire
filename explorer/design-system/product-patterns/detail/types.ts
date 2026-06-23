import type { ReactNode } from "react";

export type DetailMetaProvenance = "explicit" | "inherited";

export interface DetailMetaBadge {
  key: string;
  value: string;
  provenance: DetailMetaProvenance;
}

export interface DetailRelationEndpointData {
  id: string;
  label: string;
  kind: string;
  elementType?: string;
  typeFamily?: string;
  href: string | null;
  external: boolean;
}

export interface DetailRelationItem {
  id?: string;
  label: string;
  target: DetailRelationEndpointData;
}

export interface DetailReusedContractContextItem {
  id: string;
  targetId: string;
  kind: string;
  resourceKind?: string;
  label: string;
  elementType?: string;
  typeFamily?: string;
  href: string | null;
  external: boolean;
}

export interface DetailConceptReferenceItem {
  id: string;
  label: string;
  iri: string;
  elementId?: string;
  matchLabels?: string[];
  ontologyNodeId?: string;
  ontologyLabel?: string;
}

export interface DetailResourceTarget {
  id: string;
  href: string;
  kind: string;
  label: string;
}

export type OpenElementHandler = (id: string) => void;
export type OpenConceptReferenceHandler = (reference: DetailConceptReferenceItem) => void;
export type OpenResourceHandler = (href: string, target: DetailResourceTarget) => void;
export type OpenSourceHandler = (href: string) => void;
export type ElementDetailContentSlot = ReactNode;
