import { useMemo, useState } from "react";
import { Box, Card, Code, Flex, Grid, TextField, Text, Badge } from "@radix-ui/themes";
import { MagnifyingGlassIcon } from "@radix-ui/react-icons";
import { useStore } from "../store/StoreContext";
import { ViewFrame } from "./ViewFrame";
import { MarkdownContent } from "../components/MarkdownContent";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";
import { ExplorerWorkspaceToolbar } from "../components/ExplorerWorkspaceToolbar";
import {
  SEARCH_KINDS,
  useExplorerUiState,
  type SearchKind,
} from "../components/ExplorerUiState";

/*
 * Search view. Filters the Project Store search documents, which carry enough
 * normalized ids to route to element/file/resource/ontology/trace/coverage
 * detail without rebuilding view-local indexes from HTML text.
 */
export function SearchView({
  initialQuery,
  onOpenElement,
}: {
  initialQuery: string | null;
  onOpenElement: (id: string) => void;
} & Partial<ExplorerViewProps>) {
  const { store, elementById } = useStore();
  const { searchKinds } = useExplorerUiState();
  const [query, setQuery] = useState(initialQuery ?? "");

  const results = useMemo(() => {
    const q = query.trim().toLowerCase();
    const byKind = store.search.filter((d) => {
      const kind = normalizeSearchKind(d.kind);
      return kind ? searchKinds.has(kind) : true;
    });
    if (!q) return byKind.slice(0, 50);
    return byKind
      .filter(
        (d) =>
          d.title.toLowerCase().includes(q) || d.text.toLowerCase().includes(q),
      )
      .slice(0, 100);
  }, [query, searchKinds, store.search]);

  return (
    <ViewFrame testId="search">
      <Grid columns={{ initial: "1fr" }} className="explorer-route explorer-route-single">
        <Box className="explorer-document-panel">
          <ExplorerWorkspaceToolbar ariaLabel="Search controls">
            <div className="explorer-active-controls">
              <Text size="2" weight="bold">
                Search
              </Text>
              <Badge color="gray">{results.length} results</Badge>
            </div>
            <TextField.Root
              size="2"
              placeholder="Search elements, files, resources, ontology terms..."
              value={query}
              onChange={(e) => setQuery(e.target.value)}
            >
              <TextField.Slot>
                <MagnifyingGlassIcon />
              </TextField.Slot>
            </TextField.Root>
          </ExplorerWorkspaceToolbar>
          <Flex direction="column" gap="2">
            {results.map((d) => {
              const element = d.kind === "element" ? elementById(d.id) : undefined;
              return (
                <Card key={`${d.kind}:${d.id}`} variant="surface" className="explorer-card">
                  <button
                    type="button"
                    className="w-full text-left"
                    onClick={() => d.kind === "element" && onOpenElement(d.id)}
                  >
                    <Flex align="center" gap="2">
                      <Badge color="gray">{d.kind}</Badge>
                      <Text size="2" weight="medium">
                        {d.title}
                      </Text>
                    </Flex>
                  </button>
                  <Code color="gray" className="mt-1 block">
                    {d.route}
                  </Code>
                  {element && (
                    <MarkdownContent
                      markdown={element.content}
                      sourceFilePath={element.file_path}
                      sourceAnchor={element.source_anchor}
                      variant="preview"
                    />
                  )}
                </Card>
              );
            })}
            {results.length === 0 && <Text color="gray">No matches.</Text>}
          </Flex>
        </Box>
      </Grid>
    </ViewFrame>
  );
}

function normalizeSearchKind(kind: string): SearchKind | null {
  const normalized = kind.toLowerCase();
  return SEARCH_KINDS.includes(normalized as SearchKind) ? (normalized as SearchKind) : null;
}
