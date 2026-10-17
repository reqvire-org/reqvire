import { ReportRouteLayout, CoverageHeader, CoverageControls, CoverageKpiGrid, CoverageKpiCard, CoverageGrid, CoverageBarList, LabeledCoverageBarFrame, CoverageBarFrame, CoverageBreakdownFrame, CoverageBreakdownPie, CoverageLegendRow, CoverageCapabilityList, CoverageCapabilityRow, CoverageSourceRow, CoverageGapGrid, CoverageGapListFrame, CoverageGapRows, CoverageGapRowButton, CoverageMoreButton, CoverageEmptyState, CoverageEmptyNote, ReportEmptyNote, TraceReportPanel, TraceReportContent, TraceFileGroup, TraceFileHeader, TraceVerificationList, TraceVerificationCard, TraceVerificationHeader, TraceVerificationTitleButton, TraceTreeCountBadge, TraceVerificationMeta, TraceRowsFrame, TraceRollupDiagramShell, TraceRollupPlaceholder } from "@ds";
import { ShowcaseRegion } from "@ds/review";
import { useState } from "preact/hooks";

import { fixture } from "./ReportParts.fixture";
export const scenario = {"title": "ReportParts", covers: ["ReportRouteLayout", "CoverageHeader", "CoverageControls", "CoverageKpiGrid", "CoverageKpiCard", "CoverageGrid", "CoverageBarList", "LabeledCoverageBarFrame", "CoverageBarFrame", "CoverageBreakdownFrame", "CoverageBreakdownPie", "CoverageLegendRow", "CoverageCapabilityList", "CoverageCapabilityRow", "CoverageSourceRow", "CoverageGapGrid", "CoverageGapListFrame", "CoverageGapRows", "CoverageGapRowButton", "CoverageMoreButton", "CoverageEmptyState", "CoverageEmptyNote", "ReportEmptyNote", "TraceReportPanel", "TraceReportContent", "TraceFileGroup", "TraceFileHeader", "TraceVerificationList", "TraceVerificationCard", "TraceVerificationHeader", "TraceVerificationTitleButton", "TraceTreeCountBadge", "TraceVerificationMeta", "TraceRowsFrame", "TraceRollupDiagramShell", "TraceRollupPlaceholder"]};
export default function Scenario() {
const [scope,setScope] = useState("whole"); const [expanded,setExpanded] = useState(true);
return <ShowcaseRegion regions={["page"]}><h1>{fixture.title}</h1>
<ReportRouteLayout><CoverageHeader title="Release coverage" eyebrow="Whole model" />
<CoverageControls scope={scope} onScopeChange={setScope} scopes={[...fixture.scopes]} />
<CoverageKpiGrid><CoverageKpiCard label="Verified" detail="9 of 12 requirements" shown="75%" ringPercent={75} token="--success" /></CoverageKpiGrid>
<CoverageGrid><CoverageBarList><LabeledCoverageBarFrame label="Verification" value="75%"><CoverageBarFrame value={75} token="--success" /></LabeledCoverageBarFrame></CoverageBarList>
<CoverageBreakdownFrame><CoverageBreakdownPie segments={fixture.segments} /><CoverageLegendRow label="Verified" value={9} type="requirement" /></CoverageBreakdownFrame></CoverageGrid>
<CoverageCapabilityList><CoverageCapabilityRow name="Release reporting"><CoverageSourceRow label="Direct evidence" value={9}><p>Coverage checks are linked to requirements.</p></CoverageSourceRow></CoverageCapabilityRow></CoverageCapabilityList>
<CoverageGapGrid><CoverageGapListFrame title="Unverified" count={3}><CoverageGapRows><CoverageGapRowButton type="requirement" family="requirement" title="Complete release evidence" file="Requirements.md" typeLabel="Requirement" /></CoverageGapRows><CoverageMoreButton>Show all gaps</CoverageMoreButton></CoverageGapListFrame></CoverageGapGrid>
<CoverageEmptyState title="No matching requirements"><CoverageEmptyNote>Choose a different scope.</CoverageEmptyNote></CoverageEmptyState>
<ReportEmptyNote>No other reports in this scope.</ReportEmptyNote>
<TraceReportPanel><TraceReportContent><TraceFileGroup><TraceFileHeader file="Verifications.md" countLabel="1 verification" />
<TraceVerificationList><TraceVerificationCard><TraceVerificationHeader><TraceVerificationTitleButton onClick={() => setExpanded(!expanded)}>Release evidence verification</TraceVerificationTitleButton><TraceTreeCountBadge>12</TraceTreeCountBadge></TraceVerificationHeader><TraceVerificationMeta rows={fixture.rows} />
<TraceRowsFrame>{expanded ? <TraceRollupDiagramShell><TraceRollupPlaceholder>Choose a verification to load its trace.</TraceRollupPlaceholder></TraceRollupDiagramShell> : <p>Collapsed trace.</p>}</TraceRowsFrame></TraceVerificationCard></TraceVerificationList></TraceFileGroup></TraceReportContent></TraceReportPanel></ReportRouteLayout>
</ShowcaseRegion>;
}
