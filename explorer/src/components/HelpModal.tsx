import { Dialog, Flex, Heading, Text } from "@radix-ui/themes";

/*
 * On-demand view help modal opened from the Explorer tool rail.
 * Explanations for the primary Explorer views live here rather than on the
 * first viewport of each page.
 */
const HELP_ENTRIES: { title: string; body: string }[] = [
  {
    title: "Model",
    body: "Project model browser with List, Grid, Sunburst, and Icicle modes over files, folders, and modeled elements.",
  },
  {
    title: "Knowledge Graph",
    body: "Actual parsed project graph: elements, relation facts, attachments, concept references, governance, and source locations.",
  },
  {
    title: "Traces",
    body: "Verification trace paths and requirement/capability trace summaries.",
  },
  {
    title: "Ontologies",
    body: "OWL-aware model viewer for ontology classes, individuals, SHACL shapes, and slots/facets.",
  },
];

export function HelpModal({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content maxWidth="520px">
        <Dialog.Title>Explorer views</Dialog.Title>
        <Dialog.Description size="2" color="gray" mb="3">
          Each view reads from the same browser-local Project Store.
        </Dialog.Description>
        <Flex direction="column" gap="3">
          {HELP_ENTRIES.map((entry) => (
            <div key={entry.title}>
              <Heading as="h3" size="2">
                {entry.title}
              </Heading>
              <Text as="p" size="2" color="gray">
                {entry.body}
              </Text>
            </div>
          ))}
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}
