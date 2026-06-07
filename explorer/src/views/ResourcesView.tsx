import { Box, Card, Code, Flex, Grid, Heading, Link, Text, Badge } from "@radix-ui/themes";
import { useStore } from "../store/StoreContext";
import { ViewFrame } from "./ViewFrame";
import type { ExplorerViewProps } from "../components/ExplorerViewProps";

/*
 * Resources view (secondary / report artifact, not primary navigation).
 * Lists modeled resource and evidence-file targets from the Project Store,
 * kept distinct from browsable source `files`.
 */
export function ResourcesView(_: Partial<ExplorerViewProps> = {}) {
  const { store } = useStore();
  return (
    <ViewFrame testId="resources">
      <Grid columns={{ initial: "1fr", lg: "minmax(0, 1fr) 390px" }} className="explorer-route">
        <Box className="explorer-document-panel">
        <Flex align="center" gap="2" mb="3" wrap="wrap">
          <Heading as="h1" size="4" className="explorer-panel-title">
            Resources
          </Heading>
          <Badge color="gray">{store.resources.length} resources</Badge>
        </Flex>
        <Flex direction="column" gap="2">
          {store.resources.map((r) => (
            <Card key={r.id} variant="surface" className="explorer-card">
              <Flex align="center" gap="2" wrap="wrap">
                <Badge color="gray">{r.kind}</Badge>
                {r.external_url ? (
                  <Link href={r.external_url} target="_blank" rel="noreferrer" size="2">
                    {r.display}
                  </Link>
                ) : (
                  <Code>{r.target}</Code>
                )}
                {r.relation_types.length > 0 && (
                  <Text size="1" color="gray">
                    via {r.relation_types.join(", ")}
                  </Text>
                )}
              </Flex>
            </Card>
          ))}
          {store.resources.length === 0 && <Text color="gray">No resources in store.</Text>}
        </Flex>
        </Box>
        <Box className="graph-sidebar">
          <div className="graph-inspector-header">
            <Heading as="h2" size="3">Resource Inspector</Heading>
          </div>
          <div className="graph-inspector-body">
            <Text size="2" color="gray">
              Resource rows keep file targets, external URLs, attachments, and satisfaction evidence separate from source-file navigation.
            </Text>
          </div>
        </Box>
      </Grid>
    </ViewFrame>
  );
}
