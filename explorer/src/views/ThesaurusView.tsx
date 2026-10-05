import { useEffect } from "react";
import { ThesaurusExplorer } from "@ds";
import { useStore } from "../store/StoreContext";
import { useExplorerUiState } from "../state/ExplorerUiState";
import { prepareThesaurus } from "../lib/thesaurus";

export function ThesaurusView({ onOpenElement }: { onOpenElement?: (id: string) => void }) {
  const { store } = useStore();
  const ui = useExplorerUiState();
  const { thesaurusSelectionId, setThesaurusSelectionId } = ui;
  const concepts = prepareThesaurus(store.thesaurus).concepts;
  const selectedId = thesaurusSelectionId;

  useEffect(() => {
    if (thesaurusSelectionId && concepts.some((concept) => concept.id === thesaurusSelectionId)) return;
    setThesaurusSelectionId(concepts[0]?.id ?? null);
  }, [concepts, setThesaurusSelectionId, thesaurusSelectionId]);

  return (
    <ThesaurusExplorer
      concepts={concepts}
      selectedId={selectedId}
      onSelectConcept={setThesaurusSelectionId}
      onOpenConcept={(id) => onOpenElement?.(id)}
    />
  );
}
