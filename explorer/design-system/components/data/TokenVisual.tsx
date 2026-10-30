import { type HTMLAttributes, type ComponentChildren } from "preact";
import { css, cx } from "@linaria/atomic";
import type { DesignSystemColorToken } from "../../palette";

export interface TokenSwatchProps extends Omit<(HTMLAttributes<HTMLSpanElement> & { className?: string }), "style"> {
  colorToken: DesignSystemColorToken;
  variant?: "inline" | "sample";
}

export interface DonutMeterProps extends Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "style"> {
  percent: number;
  colorToken: DesignSystemColorToken;
  children?: ComponentChildren;
}

export interface ConicSwatchSegment {
  value: number;
  colorToken: DesignSystemColorToken;
}

export interface ConicSwatchProps extends Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "style"> {
  segments: readonly ConicSwatchSegment[];
}

export interface BarMeterFillProps extends Omit<(HTMLAttributes<HTMLSpanElement> & { className?: string }), "style"> {
  value: number;
  colorToken: DesignSystemColorToken;
}

export interface SegmentedMeterProps extends Omit<(HTMLAttributes<HTMLDivElement> & { className?: string }), "style"> {
  segments: readonly { label: string; value: number; colorToken: DesignSystemColorToken }[];
}

const tokenSwatchUX = css`
  display: inline-flex;
  width: var(--ds-token-swatch-size, var(--space-6));
  height: var(--ds-token-swatch-size, var(--space-6));
  flex: none;
  border-radius: var(--ds-token-swatch-radius, var(--radius-xs));
  background: var(--ds-token-swatch-color);
  box-shadow: inset 0 0 0 var(--border-w) var(--ds-token-swatch-ring, var(--border-strong));
`;

const sampleSwatchUX = css`
  width: 100%;
  height: var(--space-20);
  border-radius: var(--radius-md);
`;

const colorTokenSkinX = css`
  &[data-color-token="--accent"],
  .ds-token-segment[data-color-token="--accent"] { --ds-token-color: var(--accent); }
  &[data-color-token="--accent-hover"],
  .ds-token-segment[data-color-token="--accent-hover"] { --ds-token-color: var(--accent-hover); }
  &[data-color-token="--accent-active"],
  .ds-token-segment[data-color-token="--accent-active"] { --ds-token-color: var(--accent-active); }
  &[data-color-token="--accent-subtle"],
  .ds-token-segment[data-color-token="--accent-subtle"] { --ds-token-color: var(--accent-subtle); }
  &[data-color-token="--accent-ring"],
  .ds-token-segment[data-color-token="--accent-ring"] { --ds-token-color: var(--accent-ring); }
  &[data-color-token="--bg-canvas"],
  .ds-token-segment[data-color-token="--bg-canvas"] { --ds-token-color: var(--bg-canvas); }
  &[data-color-token="--bg-surface"],
  .ds-token-segment[data-color-token="--bg-surface"] { --ds-token-color: var(--bg-surface); }
  &[data-color-token="--bg-raised"],
  .ds-token-segment[data-color-token="--bg-raised"] { --ds-token-color: var(--bg-raised); }
  &[data-color-token="--bg-overlay"],
  .ds-token-segment[data-color-token="--bg-overlay"] { --ds-token-color: var(--bg-overlay); }
  &[data-color-token="--bg-sunken"],
  .ds-token-segment[data-color-token="--bg-sunken"] { --ds-token-color: var(--bg-sunken); }
  &[data-color-token="--bg-hover"],
  .ds-token-segment[data-color-token="--bg-hover"] { --ds-token-color: var(--bg-hover); }
  &[data-color-token="--bg-active"],
  .ds-token-segment[data-color-token="--bg-active"] { --ds-token-color: var(--bg-active); }
  &[data-color-token="--bg-selected"],
  .ds-token-segment[data-color-token="--bg-selected"] { --ds-token-color: var(--bg-selected); }
  &[data-color-token="--success"],
  .ds-token-segment[data-color-token="--success"] { --ds-token-color: var(--success); }
  &[data-color-token="--success-bg"],
  .ds-token-segment[data-color-token="--success-bg"] { --ds-token-color: var(--success-bg); }
  &[data-color-token="--warning"],
  .ds-token-segment[data-color-token="--warning"] { --ds-token-color: var(--warning); }
  &[data-color-token="--warning-bg"],
  .ds-token-segment[data-color-token="--warning-bg"] { --ds-token-color: var(--warning-bg); }
  &[data-color-token="--danger"],
  .ds-token-segment[data-color-token="--danger"] { --ds-token-color: var(--danger); }
  &[data-color-token="--danger-bg"],
  .ds-token-segment[data-color-token="--danger-bg"] { --ds-token-color: var(--danger-bg); }
  &[data-color-token="--info"],
  .ds-token-segment[data-color-token="--info"] { --ds-token-color: var(--info); }
  &[data-color-token="--info-bg"],
  .ds-token-segment[data-color-token="--info-bg"] { --ds-token-color: var(--info-bg); }
  &[data-color-token="--text-strong"],
  .ds-token-segment[data-color-token="--text-strong"] { --ds-token-color: var(--text-strong); }
  &[data-color-token="--text-body"],
  .ds-token-segment[data-color-token="--text-body"] { --ds-token-color: var(--text-body); }
  &[data-color-token="--text-secondary"],
  .ds-token-segment[data-color-token="--text-secondary"] { --ds-token-color: var(--text-secondary); }
  &[data-color-token="--text-muted"],
  .ds-token-segment[data-color-token="--text-muted"] { --ds-token-color: var(--text-muted); }
  &[data-color-token="--text-faint"],
  .ds-token-segment[data-color-token="--text-faint"] { --ds-token-color: var(--text-faint); }
  &[data-color-token="--text-inverse"],
  .ds-token-segment[data-color-token="--text-inverse"] { --ds-token-color: var(--text-inverse); }
  &[data-color-token="--text-link"],
  .ds-token-segment[data-color-token="--text-link"] { --ds-token-color: var(--text-link); }
  &[data-color-token="--text-code"],
  .ds-token-segment[data-color-token="--text-code"] { --ds-token-color: var(--text-code); }
  &[data-color-token="--border-subtle"],
  .ds-token-segment[data-color-token="--border-subtle"] { --ds-token-color: var(--border-subtle); }
  &[data-color-token="--border-default"],
  .ds-token-segment[data-color-token="--border-default"] { --ds-token-color: var(--border-default); }
  &[data-color-token="--border-strong"],
  .ds-token-segment[data-color-token="--border-strong"] { --ds-token-color: var(--border-strong); }
  &[data-color-token="--border-focus"],
  .ds-token-segment[data-color-token="--border-focus"] { --ds-token-color: var(--border-focus); }
  &[data-color-token="--border-selected"],
  .ds-token-segment[data-color-token="--border-selected"] { --ds-token-color: var(--border-selected); }
  &[data-color-token="--color-role-capability"],
  .ds-token-segment[data-color-token="--color-role-capability"] { --ds-token-color: var(--color-role-capability); }
  &[data-color-token="--color-role-capability-tint"],
  .ds-token-segment[data-color-token="--color-role-capability-tint"] { --ds-token-color: var(--color-role-capability-tint); }
  &[data-color-token="--color-role-capability-ink"],
  .ds-token-segment[data-color-token="--color-role-capability-ink"] { --ds-token-color: var(--color-role-capability-ink); }
  &[data-color-token="--color-role-requirement"],
  .ds-token-segment[data-color-token="--color-role-requirement"] { --ds-token-color: var(--color-role-requirement); }
  &[data-color-token="--color-role-requirement-tint"],
  .ds-token-segment[data-color-token="--color-role-requirement-tint"] { --ds-token-color: var(--color-role-requirement-tint); }
  &[data-color-token="--color-role-requirement-ink"],
  .ds-token-segment[data-color-token="--color-role-requirement-ink"] { --ds-token-color: var(--color-role-requirement-ink); }
  &[data-color-token="--color-role-contract"],
  .ds-token-segment[data-color-token="--color-role-contract"] { --ds-token-color: var(--color-role-contract); }
  &[data-color-token="--color-role-contract-tint"],
  .ds-token-segment[data-color-token="--color-role-contract-tint"] { --ds-token-color: var(--color-role-contract-tint); }
  &[data-color-token="--color-role-contract-ink"],
  .ds-token-segment[data-color-token="--color-role-contract-ink"] { --ds-token-color: var(--color-role-contract-ink); }
  &[data-color-token="--color-role-semantic-contract"],
  .ds-token-segment[data-color-token="--color-role-semantic-contract"] { --ds-token-color: var(--color-role-semantic-contract); }
  &[data-color-token="--color-role-semantic-contract-tint"],
  .ds-token-segment[data-color-token="--color-role-semantic-contract-tint"] { --ds-token-color: var(--color-role-semantic-contract-tint); }
  &[data-color-token="--color-role-semantic-contract-ink"],
  .ds-token-segment[data-color-token="--color-role-semantic-contract-ink"] { --ds-token-color: var(--color-role-semantic-contract-ink); }
  &[data-color-token="--color-role-verification"],
  .ds-token-segment[data-color-token="--color-role-verification"] { --ds-token-color: var(--color-role-verification); }
  &[data-color-token="--color-role-verification-tint"],
  .ds-token-segment[data-color-token="--color-role-verification-tint"] { --ds-token-color: var(--color-role-verification-tint); }
  &[data-color-token="--color-role-verification-ink"],
  .ds-token-segment[data-color-token="--color-role-verification-ink"] { --ds-token-color: var(--color-role-verification-ink); }
  &[data-color-token="--color-role-verification-type-test"],
  .ds-token-segment[data-color-token="--color-role-verification-type-test"] { --ds-token-color: var(--color-role-verification-type-test); }
  &[data-color-token="--color-role-verification-type-formal-proof"],
  .ds-token-segment[data-color-token="--color-role-verification-type-formal-proof"] { --ds-token-color: var(--color-role-verification-type-formal-proof); }
  &[data-color-token="--color-role-verification-type-analysis"],
  .ds-token-segment[data-color-token="--color-role-verification-type-analysis"] { --ds-token-color: var(--color-role-verification-type-analysis); }
  &[data-color-token="--color-role-verification-type-inspection"],
  .ds-token-segment[data-color-token="--color-role-verification-type-inspection"] { --ds-token-color: var(--color-role-verification-type-inspection); }
  &[data-color-token="--color-role-verification-type-demonstration"],
  .ds-token-segment[data-color-token="--color-role-verification-type-demonstration"] { --ds-token-color: var(--color-role-verification-type-demonstration); }
  &[data-color-token="--color-role-ontology"],
  .ds-token-segment[data-color-token="--color-role-ontology"] { --ds-token-color: var(--color-role-ontology); }
  &[data-color-token="--color-role-ontology-tint"],
  .ds-token-segment[data-color-token="--color-role-ontology-tint"] { --ds-token-color: var(--color-role-ontology-tint); }
  &[data-color-token="--color-role-ontology-ink"],
  .ds-token-segment[data-color-token="--color-role-ontology-ink"] { --ds-token-color: var(--color-role-ontology-ink); }
  &[data-color-token="--color-role-resource"],
  .ds-token-segment[data-color-token="--color-role-resource"] { --ds-token-color: var(--color-role-resource); }
  &[data-color-token="--color-role-resource-tint"],
  .ds-token-segment[data-color-token="--color-role-resource-tint"] { --ds-token-color: var(--color-role-resource-tint); }
  &[data-color-token="--color-role-resource-ink"],
  .ds-token-segment[data-color-token="--color-role-resource-ink"] { --ds-token-color: var(--color-role-resource-ink); }
  &[data-color-token="--color-role-other"],
  .ds-token-segment[data-color-token="--color-role-other"] { --ds-token-color: var(--color-role-other); }
  &[data-color-token="--color-role-other-tint"],
  .ds-token-segment[data-color-token="--color-role-other-tint"] { --ds-token-color: var(--color-role-other-tint); }
  &[data-color-token="--color-role-other-ink"],
  .ds-token-segment[data-color-token="--color-role-other-ink"] { --ds-token-color: var(--color-role-other-ink); }
  &[data-color-token="--color-role-edge-default"],
  .ds-token-segment[data-color-token="--color-role-edge-default"] { --ds-token-color: var(--color-role-edge-default); }
  &[data-color-token="--color-role-edge-derive"],
  .ds-token-segment[data-color-token="--color-role-edge-derive"] { --ds-token-color: var(--color-role-edge-derive); }
  &[data-color-token="--color-role-edge-satisfy"],
  .ds-token-segment[data-color-token="--color-role-edge-satisfy"] { --ds-token-color: var(--color-role-edge-satisfy); }
  &[data-color-token="--color-role-edge-trace"],
  .ds-token-segment[data-color-token="--color-role-edge-trace"] { --ds-token-color: var(--color-role-edge-trace); }
  &[data-color-token="--color-role-edge-bind"],
  .ds-token-segment[data-color-token="--color-role-edge-bind"] { --ds-token-color: var(--color-role-edge-bind); }
  &[data-color-token="--color-role-rdf-class"],
  .ds-token-segment[data-color-token="--color-role-rdf-class"] { --ds-token-color: var(--color-role-rdf-class); }
  &[data-color-token="--color-role-rdf-concept"],
  .ds-token-segment[data-color-token="--color-role-rdf-concept"] { --ds-token-color: var(--color-role-rdf-concept); }
  &[data-color-token="--color-role-rdf-concept-scheme"],
  .ds-token-segment[data-color-token="--color-role-rdf-concept-scheme"] { --ds-token-color: var(--color-role-rdf-concept-scheme); }
  &[data-color-token="--color-role-concept"],
  .ds-token-segment[data-color-token="--color-role-concept"] { --ds-token-color: var(--color-role-concept); }
  &[data-color-token="--color-role-concept-ink"],
  .ds-token-segment[data-color-token="--color-role-concept-ink"] { --ds-token-color: var(--color-role-concept-ink); }
  &[data-color-token="--color-role-concept-tint"],
  .ds-token-segment[data-color-token="--color-role-concept-tint"] { --ds-token-color: var(--color-role-concept-tint); }
  &[data-color-token="--color-role-concept-scheme"],
  .ds-token-segment[data-color-token="--color-role-concept-scheme"] { --ds-token-color: var(--color-role-concept-scheme); }
  &[data-color-token="--color-role-concept-scheme-ink"],
  .ds-token-segment[data-color-token="--color-role-concept-scheme-ink"] { --ds-token-color: var(--color-role-concept-scheme-ink); }
  &[data-color-token="--color-role-concept-scheme-tint"],
  .ds-token-segment[data-color-token="--color-role-concept-scheme-tint"] { --ds-token-color: var(--color-role-concept-scheme-tint); }
  &[data-color-token="--color-role-concept-reference"],
  .ds-token-segment[data-color-token="--color-role-concept-reference"] { --ds-token-color: var(--color-role-concept-reference); }
  &[data-color-token="--color-role-rdf-objprop"],
  .ds-token-segment[data-color-token="--color-role-rdf-objprop"] { --ds-token-color: var(--color-role-rdf-objprop); }
  &[data-color-token="--color-role-rdf-dtprop"],
  .ds-token-segment[data-color-token="--color-role-rdf-dtprop"] { --ds-token-color: var(--color-role-rdf-dtprop); }
  &[data-color-token="--color-role-rdf-rdfprop"],
  .ds-token-segment[data-color-token="--color-role-rdf-rdfprop"] { --ds-token-color: var(--color-role-rdf-rdfprop); }
  &[data-color-token="--color-role-rdf-individual"],
  .ds-token-segment[data-color-token="--color-role-rdf-individual"] { --ds-token-color: var(--color-role-rdf-individual); }
  &[data-color-token="--color-role-rdf-datatype"],
  .ds-token-segment[data-color-token="--color-role-rdf-datatype"] { --ds-token-color: var(--color-role-rdf-datatype); }
  &[data-color-token="--color-role-rdf-restriction"],
  .ds-token-segment[data-color-token="--color-role-rdf-restriction"] { --ds-token-color: var(--color-role-rdf-restriction); }
  &[data-color-token="--color-role-rdf-classexpr"],
  .ds-token-segment[data-color-token="--color-role-rdf-classexpr"] { --ds-token-color: var(--color-role-rdf-classexpr); }
  &[data-color-token="--color-role-rdf-nodeshape"],
  .ds-token-segment[data-color-token="--color-role-rdf-nodeshape"] { --ds-token-color: var(--color-role-rdf-nodeshape); }
  &[data-color-token="--color-role-rdf-propshape"],
  .ds-token-segment[data-color-token="--color-role-rdf-propshape"] { --ds-token-color: var(--color-role-rdf-propshape); }
  &[data-color-token="--color-role-rdf-resource"],
  .ds-token-segment[data-color-token="--color-role-rdf-resource"] { --ds-token-color: var(--color-role-rdf-resource); }
  &[data-color-token="--color-role-rdf-shacl"],
  .ds-token-segment[data-color-token="--color-role-rdf-shacl"] { --ds-token-color: var(--color-role-rdf-shacl); }

  --ds-token-swatch-color: var(--ds-token-color);
  --ds-donut-color: var(--ds-token-color);
  --ds-bar-color: var(--ds-token-color);
`;

const donutUX = css`
  display: grid;
  place-items: center;

  .ds-donut__svg {
    grid-area: 1 / 1;
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }

  .ds-donut__track {
    fill: none;
    stroke: var(--bg-sunken);
    stroke-width: var(--space-5);
  }

  .ds-donut__fill {
    fill: none;
    stroke: var(--ds-donut-color);
    stroke-width: var(--space-5);
    transition: stroke-dasharray var(--dur-fast) var(--ease-standard);
  }

  .ds-donut__content {
    grid-area: 1 / 1;
    z-index: var(--z-local-base);
  }
`;

const conicUX = css`
  display: grid;
  place-items: center;

  .ds-conic__svg {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }

  .ds-token-segment {
    fill: none;
    stroke: var(--ds-token-color);
    stroke-width: var(--space-24);
  }
`;

const barUX = css`
  display: block;

  .ds-bar__svg {
    display: block;
    width: 100%;
    height: 100%;
  }

  .ds-bar__fill {
    fill: var(--ds-bar-color);
  }
`;

const segmentedMeterUX = css`
  width: 100%;
  height: var(--space-5);
  overflow: hidden;
  border-radius: var(--radius-full);
  background: var(--bg-sunken);
  svg { display: block; width: 100%; height: 100%; }
  rect { fill: var(--ds-token-color); stroke: var(--bg-surface); stroke-width: var(--border-w); }
`;

function clampPercent(value: number) {
  return Math.max(0, Math.min(100, Number.isFinite(value) ? value : 0));
}

export function TokenSwatch({ colorToken, variant = "inline", className = "", ...props }: TokenSwatchProps) {
  return (
    <span
      className={cx("ds-token-swatch", tokenSwatchUX, colorTokenSkinX, variant === "sample" && sampleSwatchUX, className)}
      data-color-token={colorToken}
      {...props}
    />
  );
}

export function DonutMeter({
  percent,
  colorToken,
  className = "",
  children,
  ...props
}: DonutMeterProps) {
  const clamped = clampPercent(percent);
  return (
    <div className={cx("ds-donut", donutUX, colorTokenSkinX, className)} data-color-token={colorToken} {...props}>
      <svg className="ds-donut__svg" viewBox="0 0 100 100" aria-hidden="true">
        <circle className="ds-donut__track" cx="50" cy="50" r="40" />
        <circle
          className="ds-donut__fill"
          cx="50"
          cy="50"
          r="40"
          pathLength={100}
          stroke-dasharray={`${clamped} ${100 - clamped}`}
        />
      </svg>
      <span className="ds-donut__content">{children}</span>
    </div>
  );
}

export function ConicSwatch({ segments, className = "", ...props }: ConicSwatchProps) {
  const total = segments.reduce((sum, segment) => sum + Math.max(0, segment.value), 0) || 1;
  let offset = 0;
  return (
    <div className={cx("ds-conic", conicUX, colorTokenSkinX, className)} {...props}>
      <svg className="ds-conic__svg" viewBox="0 0 100 100" aria-hidden="true">
        {segments.map((segment, index) => {
          const percent = clampPercent((Math.max(0, segment.value) / total) * 100);
          const dashOffset = -offset;
          offset += percent;
          return (
            <circle
              key={`${segment.colorToken}-${index}`}
              className="ds-token-segment"
              data-color-token={segment.colorToken}
              cx="50"
              cy="50"
              r="38"
              pathLength={100}
              stroke-dasharray={`${percent} ${100 - percent}`}
              stroke-dashoffset={dashOffset}
            />
          );
        })}
      </svg>
    </div>
  );
}

export function BarMeterFill({
  value,
  colorToken,
  className = "",
  ...props
}: BarMeterFillProps) {
  const clamped = clampPercent(value);
  return (
    <span className={cx("ds-bar", barUX, colorTokenSkinX, className)} data-color-token={colorToken} {...props}>
      <svg className="ds-bar__svg" viewBox="0 0 100 1" preserveAspectRatio="none" aria-hidden="true">
        <rect className="ds-bar__fill" x="0" y="0" width={clamped} height="1" />
      </svg>
    </span>
  );
}

/** Relative shares; labels and exact counts should also be presented in the surrounding legend. */
export function SegmentedMeter({ segments, className = "", ...props }: SegmentedMeterProps) {
  const values = segments.map(segment => Number.isFinite(segment.value) ? Math.max(0, segment.value) : 0);
  const total = values.reduce((sum, value) => sum + value, 0) || 1;
  let offset = 0;
  return <div data-meter="segmented" className={cx("ds-segmented-meter", segmentedMeterUX, colorTokenSkinX, className)} {...props}>
    <svg viewBox="0 0 100 1" preserveAspectRatio="none" aria-hidden="true">
      {segments.map((segment, index) => {
        const width = values[index] / total * 100;
        const x = offset; offset += width;
        return values[index] > 0 ? <rect key={segment.label} className="ds-token-segment" data-color-token={segment.colorToken}
          data-segment-label={segment.label} data-segment-value={values[index]} x={x} y="0" width={width} height="1" vector-effect="non-scaling-stroke">
          <title>{segment.label}: {values[index]}</title>
        </rect> : null;
      })}
    </svg>
  </div>;
}
