import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";
import { type ComponentChildren } from "preact";
import { CodeBody, CodePreviewFrame, CodeToolbar, CodeFallback, DiagramBlockFrame, DocumentPanel, MarkdownFrame, RendererNotice } from "@ds";
import { fixture, CODE_SAMPLE } from "./DocumentPanel.fixture";
export const scenario = {"title":"Content Renderer Frames",covers: ["DocumentPanel","MarkdownFrame","DiagramBlockFrame","CodePreviewFrame","CodeToolbar","CodeBody","CodeFallback"]};
function Section({ title, desc, children }: {
    title: string;
    desc?: string;
    children: ComponentChildren;
}) {
    return (<ShowcaseRegion regions={["section"]} as="section">
      <ShowcaseRegion regions={["section__heading"]}>
        <ShowcaseRegion regions={["section__title"]}>{title}</ShowcaseRegion>
        {desc && <ShowcaseRegion regions={["section__desc"]}>{desc}</ShowcaseRegion>}
      </ShowcaseRegion>
      {children}
    </ShowcaseRegion>);
}
export default function Scenario() {
const [codeExpanded, setCodeExpanded] = useState(true);
const [codeWrapped, setCodeWrapped] = useState(false);
const [codeCopied, setCodeCopied] = useState(false);
return <ShowcaseRegion regions={["page"]}>
<Section title={fixture.title} desc={fixture.description}>
        <ShowcaseRegion regions={["pattern-grid", "pattern-grid--two"]}>
          <ShowcaseRegion regions={["pattern-card", "pattern-card--tall"]}>
            <DocumentPanel layout="embedded" toolbar={{
        label: "Source page",
        title: "system-model/SystemRequirements.md",
        actionHref: "#open-source",
        actionLabel: "Open source",
    }}>
              <MarkdownFrame>
                <h2>Traceability Coverage Requirement</h2>
                <p>
                  Every approved requirement exposes at least one verification relation before
                  the release coverage report is accepted.
                </p>
                <blockquote>Coverage gaps remain visible until an explicit verification target is linked.</blockquote>
              </MarkdownFrame>
              <DiagramBlockFrame>
                <svg viewBox="0 0 620 160" role="img" aria-label="Mock traceability diagram">
                  <rect x="16" y="42" width="160" height="76" rx="8" fill="var(--color-role-requirement-tint)" stroke="var(--color-role-requirement)"/>
                  <text x="96" y="76" text-anchor="middle" fill="var(--text-strong)">Requirement</text>
                  <rect x="232" y="42" width="160" height="76" rx="8" fill="var(--color-role-verification-tint)" stroke="var(--color-role-verification)"/>
                  <text x="312" y="76" text-anchor="middle" fill="var(--text-strong)">Verification</text>
                  <rect x="448" y="42" width="156" height="76" rx="8" fill="var(--color-role-resource-tint)" stroke="var(--color-role-resource)"/>
                  <text x="526" y="76" text-anchor="middle" fill="var(--text-strong)">Evidence</text>
                  <path d="M176 80 H232" stroke="var(--color-role-edge-trace)" stroke-width="3"/>
                  <path d="M392 80 H448" stroke="var(--color-role-edge-bind)" stroke-width="3"/>
                </svg>
              </DiagramBlockFrame>
              <RendererNotice title="Renderer note">
                This preview is driven by static showcase content; markdown parsing remains in the app container.
              </RendererNotice>
            </DocumentPanel>
          </ShowcaseRegion>

          <ShowcaseRegion regions={["pattern-card"]}>
            <CodePreviewFrame wrapped={codeWrapped}>
              <CodeToolbar expanded={codeExpanded} onToggleExpanded={() => setCodeExpanded((expanded) => !expanded)} kind="source" path="src/lib/coverage.ts" languageLabel="TypeScript" lineCount={4} relationTypes={["specifiedIn", "verifiedBy"]} wrapped={codeWrapped} copied={codeCopied} onToggleWrap={() => setCodeWrapped((wrapped) => !wrapped)} onCopy={() => setCodeCopied(true)}/>
              <CodeBody collapsed={!codeExpanded} lineCount={4}>
                <CodeFallback>
                  {CODE_SAMPLE}
                </CodeFallback>
              </CodeBody>
            </CodePreviewFrame>
            <RendererNotice inline tone="empty">
              Empty-state renderer notices can also appear inline.
            </RendererNotice>
          </ShowcaseRegion>
        </ShowcaseRegion>
      </Section>
</ShowcaseRegion>;
}
