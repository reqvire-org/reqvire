import { RendererNotice } from "@ds";
import { ShowcaseRegion } from "@ds/review";
import { fixture } from "./RendererNotice.fixture";

export const scenario = { title: fixture.title, covers: ["RendererNotice"] };

export default function Scenario() {
  return <ShowcaseRegion regions={["page"]}>
    <ShowcaseRegion regions={["section"]} as="section">
      <ShowcaseRegion regions={["section__heading"]}>
        <ShowcaseRegion regions={["section__title"]}>{fixture.title}</ShowcaseRegion>
        <ShowcaseRegion regions={["section__desc"]}>{fixture.description}</ShowcaseRegion>
      </ShowcaseRegion>
      <RendererNotice title="Renderer note">{fixture.note}</RendererNotice>
      <RendererNotice tone="empty">{fixture.empty}</RendererNotice>
      <RendererNotice inline tone="empty">{fixture.empty}</RendererNotice>
    </ShowcaseRegion>
  </ShowcaseRegion>;
}
