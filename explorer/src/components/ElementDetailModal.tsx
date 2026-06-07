import { useMemo } from "react";
import {
  Badge,
  Box,
  Button,
  Code,
  DataList,
  Dialog,
  Flex,
  Heading,
  Link,
  ScrollArea,
  Separator,
  Text,
} from "@radix-ui/themes";
import { ExternalLinkIcon } from "@radix-ui/react-icons";
import { useStore } from "../store/StoreContext";
import { routeForElement } from "../router/routes";
import { MarkdownContent } from "./MarkdownContent";

/*
 * Element-detail modal.
 *
 * Renders as an in-shell scrollable dialog over the active Explorer view,
 * backed by Project Store element records. Shows name, type, source file/anchor,
 * metadata, governance, content, relations, attachments, and concept references.
 * Provides a SECONDARY source-page action (the exported source anchor); that
 * action is not the primary navigation target. Closing returns to the
 * underlying route (handled by the caller via onClose).
 */
export function ElementDetailModal({
  identifier,
  onClose,
  onOpenElement,
}: {
  identifier: string | null;
  onClose: () => void;
  onOpenElement: (id: string) => void;
}) {
  const { store, elementById } = useStore();
  const element = identifier ? elementById(identifier) : undefined;

  const { outgoing, incoming, attachments, conceptRefs } = useMemo(() => {
    if (!identifier) {
      return { outgoing: [], incoming: [], attachments: [], conceptRefs: [] };
    }
    return {
      outgoing: store.relations.filter((r) => r.source_id === identifier),
      incoming: store.relations.filter((r) => r.target_id === identifier),
      attachments: store.attachments.filter((a) => a.source_id === identifier),
      conceptRefs: store.concept_refs.filter((c) => c.source_id === identifier),
    };
  }, [identifier, store]);

  const open = identifier !== null;

  return (
    <Dialog.Root open={open} onOpenChange={(v) => !v && onClose()}>
      <Dialog.Content maxWidth="760px" className="max-h-[85vh]">
        {!element ? (
          <Box>
            <Dialog.Title>Element not found</Dialog.Title>
            <Text as="p" size="2" color="gray">
              No Project Store element matches{" "}
              <Code>{identifier ?? ""}</Code>.
            </Text>
          </Box>
        ) : (
          <>
            <Flex align="center" gap="2" wrap="wrap" mb="1">
              <Badge color="gray">{element.element_type}</Badge>
              <Dialog.Title className="m-0">{element.name}</Dialog.Title>
            </Flex>
            <Dialog.Description size="1" color="gray" mb="3">
              <Code>{element.id}</Code>
            </Dialog.Description>

            <ScrollArea type="auto" scrollbars="vertical" className="max-h-[60vh] pr-3">
              <Flex direction="column" gap="4">
                <DataList.Root size="2">
                  <DataList.Item>
                    <DataList.Label>Type family</DataList.Label>
                    <DataList.Value>{element.type_family}</DataList.Value>
                  </DataList.Item>
                  <DataList.Item>
                    <DataList.Label>Source file</DataList.Label>
                    <DataList.Value>
                      <Code>
                        {element.file_path}:{element.line_number}
                      </Code>
                    </DataList.Value>
                  </DataList.Item>
                  {Object.entries(element.metadata).map(([k, v]) => (
                    <DataList.Item key={`meta-${k}`}>
                      <DataList.Label>{k}</DataList.Label>
                      <DataList.Value>{v}</DataList.Value>
                    </DataList.Item>
                  ))}
                  {Object.entries(element.governance).map(([k, v]) => (
                    <DataList.Item key={`gov-${k}`}>
                      <DataList.Label>{k}</DataList.Label>
                      <DataList.Value>{v}</DataList.Value>
                    </DataList.Item>
                  ))}
                </DataList.Root>

                <Section title="Content">
                  <MarkdownContent
                    markdown={element.content}
                    sourceFilePath={element.file_path}
                    sourceAnchor={element.source_anchor}
                  />
                </Section>

                <RelationList
                  title="Outgoing relations"
                  relations={outgoing.map((r) => ({
                    label: r.relation_type,
                    targetId: r.target_id,
                    targetKind: r.target_kind,
                    generated: r.generated_opposite,
                  }))}
                  onOpenElement={onOpenElement}
                />
                <RelationList
                  title="Incoming relations"
                  relations={incoming.map((r) => ({
                    label: r.relation_type,
                    targetId: r.source_id,
                    targetKind: "element",
                    generated: r.generated_opposite,
                  }))}
                  onOpenElement={onOpenElement}
                />

                {attachments.length > 0 && (
                  <Section title="Attachments">
                    <Flex direction="column" gap="1">
                      {attachments.map((a) => (
                        <Flex key={a.id} gap="2" align="center">
                          <Badge color="gray" variant="soft">
                            {a.target_kind}
                          </Badge>
                          <Code>{a.target}</Code>
                        </Flex>
                      ))}
                    </Flex>
                  </Section>
                )}

                {conceptRefs.length > 0 && (
                  <Section title="Concept references">
                    <Flex direction="column" gap="1">
                      {conceptRefs.map((c) => (
                        <Flex key={c.id} gap="2" align="center">
                          <Text size="2">{c.label}</Text>
                          <Code>{c.iri}</Code>
                        </Flex>
                      ))}
                    </Flex>
                  </Section>
                )}
              </Flex>
            </ScrollArea>

            <Separator size="4" my="3" />
            <Flex justify="between" align="center" gap="2">
              <Link
                href={element.source_anchor}
                size="2"
                color="gray"
                target="_blank"
                rel="noreferrer"
              >
                <Flex align="center" gap="1">
                  <ExternalLinkIcon /> Open source page
                </Flex>
              </Link>
              <Dialog.Close>
                <Button variant="soft" color="gray">
                  Close
                </Button>
              </Dialog.Close>
            </Flex>
          </>
        )}
      </Dialog.Content>
    </Dialog.Root>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <Box>
      <Heading as="h3" size="2" mb="1">
        {title}
      </Heading>
      {children}
    </Box>
  );
}

function RelationList({
  title,
  relations,
  onOpenElement,
}: {
  title: string;
  relations: { label: string; targetId: string; targetKind: string; generated: boolean }[];
  onOpenElement: (id: string) => void;
}) {
  if (relations.length === 0) return null;
  return (
    <Section title={title}>
      <Flex direction="column" gap="1">
        {relations.map((r, i) => (
          <Flex key={`${r.label}-${r.targetId}-${i}`} gap="2" align="center" wrap="wrap">
            <Badge color="gray" variant="soft">
              {r.label}
            </Badge>
            {r.targetKind === "element" ? (
              <Link
                href={routeForElement(r.targetId)}
                size="2"
                onClick={(e) => {
                  e.preventDefault();
                  onOpenElement(r.targetId);
                }}
              >
                {r.targetId}
              </Link>
            ) : (
              <Code>{r.targetId}</Code>
            )}
            {r.generated && (
              <Text size="1" color="gray">
                (generated)
              </Text>
            )}
          </Flex>
        ))}
      </Flex>
    </Section>
  );
}
