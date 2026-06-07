import { Callout, Container, Flex, Heading, Text, Code } from "@radix-ui/themes";
import { ExclamationTriangleIcon } from "@radix-ui/react-icons";

/*
 * Visible fail-closed diagnostic shown when the Project Store seed is missing,
 * malformed, or incompatible. The Explorer must not render partial views
 * without a valid seed.
 */
export function MissingStoreNotice({
  reason,
  detail,
}: {
  reason: string;
  detail?: string;
}) {
  return (
    <Container size="2" px="4" py="8">
      <Flex direction="column" gap="4">
        <Heading size="6">Reqvire Explorer</Heading>
        <Callout.Root color="red" role="alert">
          <Callout.Icon>
            <ExclamationTriangleIcon />
          </Callout.Icon>
          <Callout.Text>{reason}</Callout.Text>
        </Callout.Root>
        {detail && (
          <Text size="2" color="gray">
            <Code>{detail}</Code>
          </Text>
        )}
        <Text size="2" color="gray">
          The Explorer shell loads a browser-local Project Store seed produced by
          a Reqvire HTML export. Open this page from an export, or run{" "}
          <Code>npm run dev</Code> in <Code>explorer/</Code> to browse the shell
          with fixture data.
        </Text>
      </Flex>
    </Container>
  );
}
