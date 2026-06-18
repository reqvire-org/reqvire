import { useState } from "react";
import { OntologyNodeDetailDialog } from "@ds";
import { useStore } from "../store/StoreContext";

export function OntologyNodeDetailModal({
  nodeId,
  onClose,
}: {
  nodeId: string | null;
  onClose: () => void;
}) {
  const { store } = useStore();
  const node = nodeId
    ? (store.ontology.graph_data?.nodes ?? []).find((candidate) => candidate.id === nodeId)
    : undefined;
  const graphNodes = store.ontology.graph_data?.nodes ?? [];
  const [copiedUri, setCopiedUri] = useState(false);

  return (
    <OntologyNodeDetailDialog
      open={nodeId !== null}
      onOpenChange={(open) => {
        if (!open) onClose();
      }}
      node={node}
      nodes={graphNodes}
      missingNodeId={nodeId}
      copiedUri={copiedUri}
      onCopyUri={(uri) => {
        void navigator.clipboard?.writeText(uri);
        setCopiedUri(true);
        window.setTimeout(() => setCopiedUri(false), 1400);
      }}
      onOpenSource={(source) => {
        if (source.link) {
          window.location.hash = source.link;
          onClose();
        }
      }}
    />
  );
}
