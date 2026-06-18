import type { HTMLAttributes, ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import type { DesignSystemColorToken } from "../../palette";

export interface TokenSwatchProps extends Omit<HTMLAttributes<HTMLSpanElement>, "style"> {
  colorToken: DesignSystemColorToken;
}

export interface DonutMeterProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  percent: number;
  colorToken: DesignSystemColorToken;
  children?: ReactNode;
}

export interface ConicSwatchSegment {
  value: number;
  colorToken: DesignSystemColorToken;
}

export interface ConicSwatchProps extends Omit<HTMLAttributes<HTMLDivElement>, "style"> {
  segments: readonly ConicSwatchSegment[];
}

export interface BarMeterFillProps extends Omit<HTMLAttributes<HTMLSpanElement>, "style"> {
  value: number;
  colorToken: DesignSystemColorToken;
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
  &[data-color-token="--success-tint"],
  .ds-token-segment[data-color-token="--success-tint"] { --ds-token-color: var(--success-tint); }
  &[data-color-token="--warning"],
  .ds-token-segment[data-color-token="--warning"] { --ds-token-color: var(--warning); }
  &[data-color-token="--warning-tint"],
  .ds-token-segment[data-color-token="--warning-tint"] { --ds-token-color: var(--warning-tint); }
  &[data-color-token="--danger"],
  .ds-token-segment[data-color-token="--danger"] { --ds-token-color: var(--danger); }
  &[data-color-token="--danger-tint"],
  .ds-token-segment[data-color-token="--danger-tint"] { --ds-token-color: var(--danger-tint); }
  &[data-color-token="--info"],
  .ds-token-segment[data-color-token="--info"] { --ds-token-color: var(--info); }
  &[data-color-token="--info-tint"],
  .ds-token-segment[data-color-token="--info-tint"] { --ds-token-color: var(--info-tint); }
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
  &[data-color-token="--capability"],
  .ds-token-segment[data-color-token="--capability"] { --ds-token-color: var(--capability); }
  &[data-color-token="--capability-tint"],
  .ds-token-segment[data-color-token="--capability-tint"] { --ds-token-color: var(--capability-tint); }
  &[data-color-token="--capability-ink"],
  .ds-token-segment[data-color-token="--capability-ink"] { --ds-token-color: var(--capability-ink); }
  &[data-color-token="--requirement"],
  .ds-token-segment[data-color-token="--requirement"] { --ds-token-color: var(--requirement); }
  &[data-color-token="--requirement-tint"],
  .ds-token-segment[data-color-token="--requirement-tint"] { --ds-token-color: var(--requirement-tint); }
  &[data-color-token="--requirement-ink"],
  .ds-token-segment[data-color-token="--requirement-ink"] { --ds-token-color: var(--requirement-ink); }
  &[data-color-token="--contract"],
  .ds-token-segment[data-color-token="--contract"] { --ds-token-color: var(--contract); }
  &[data-color-token="--contract-tint"],
  .ds-token-segment[data-color-token="--contract-tint"] { --ds-token-color: var(--contract-tint); }
  &[data-color-token="--contract-ink"],
  .ds-token-segment[data-color-token="--contract-ink"] { --ds-token-color: var(--contract-ink); }
  &[data-color-token="--semantic-contract"],
  .ds-token-segment[data-color-token="--semantic-contract"] { --ds-token-color: var(--semantic-contract); }
  &[data-color-token="--semantic-contract-tint"],
  .ds-token-segment[data-color-token="--semantic-contract-tint"] { --ds-token-color: var(--semantic-contract-tint); }
  &[data-color-token="--semantic-contract-ink"],
  .ds-token-segment[data-color-token="--semantic-contract-ink"] { --ds-token-color: var(--semantic-contract-ink); }
  &[data-color-token="--verification"],
  .ds-token-segment[data-color-token="--verification"] { --ds-token-color: var(--verification); }
  &[data-color-token="--verification-tint"],
  .ds-token-segment[data-color-token="--verification-tint"] { --ds-token-color: var(--verification-tint); }
  &[data-color-token="--verification-ink"],
  .ds-token-segment[data-color-token="--verification-ink"] { --ds-token-color: var(--verification-ink); }
  &[data-color-token="--ontology"],
  .ds-token-segment[data-color-token="--ontology"] { --ds-token-color: var(--ontology); }
  &[data-color-token="--ontology-tint"],
  .ds-token-segment[data-color-token="--ontology-tint"] { --ds-token-color: var(--ontology-tint); }
  &[data-color-token="--ontology-ink"],
  .ds-token-segment[data-color-token="--ontology-ink"] { --ds-token-color: var(--ontology-ink); }
  &[data-color-token="--resource"],
  .ds-token-segment[data-color-token="--resource"] { --ds-token-color: var(--resource); }
  &[data-color-token="--resource-tint"],
  .ds-token-segment[data-color-token="--resource-tint"] { --ds-token-color: var(--resource-tint); }
  &[data-color-token="--resource-ink"],
  .ds-token-segment[data-color-token="--resource-ink"] { --ds-token-color: var(--resource-ink); }
  &[data-color-token="--other"],
  .ds-token-segment[data-color-token="--other"] { --ds-token-color: var(--other); }
  &[data-color-token="--other-tint"],
  .ds-token-segment[data-color-token="--other-tint"] { --ds-token-color: var(--other-tint); }
  &[data-color-token="--other-ink"],
  .ds-token-segment[data-color-token="--other-ink"] { --ds-token-color: var(--other-ink); }
  &[data-color-token="--edge-default"],
  .ds-token-segment[data-color-token="--edge-default"] { --ds-token-color: var(--edge-default); }
  &[data-color-token="--edge-derive"],
  .ds-token-segment[data-color-token="--edge-derive"] { --ds-token-color: var(--edge-derive); }
  &[data-color-token="--edge-satisfy"],
  .ds-token-segment[data-color-token="--edge-satisfy"] { --ds-token-color: var(--edge-satisfy); }
  &[data-color-token="--edge-trace"],
  .ds-token-segment[data-color-token="--edge-trace"] { --ds-token-color: var(--edge-trace); }
  &[data-color-token="--edge-reuse"],
  .ds-token-segment[data-color-token="--edge-reuse"] { --ds-token-color: var(--edge-reuse); }
  &[data-color-token="--rdf-class"],
  .ds-token-segment[data-color-token="--rdf-class"] { --ds-token-color: var(--rdf-class); }
  &[data-color-token="--rdf-objprop"],
  .ds-token-segment[data-color-token="--rdf-objprop"] { --ds-token-color: var(--rdf-objprop); }
  &[data-color-token="--rdf-dtprop"],
  .ds-token-segment[data-color-token="--rdf-dtprop"] { --ds-token-color: var(--rdf-dtprop); }
  &[data-color-token="--rdf-rdfprop"],
  .ds-token-segment[data-color-token="--rdf-rdfprop"] { --ds-token-color: var(--rdf-rdfprop); }
  &[data-color-token="--rdf-individual"],
  .ds-token-segment[data-color-token="--rdf-individual"] { --ds-token-color: var(--rdf-individual); }
  &[data-color-token="--rdf-datatype"],
  .ds-token-segment[data-color-token="--rdf-datatype"] { --ds-token-color: var(--rdf-datatype); }
  &[data-color-token="--rdf-restriction"],
  .ds-token-segment[data-color-token="--rdf-restriction"] { --ds-token-color: var(--rdf-restriction); }
  &[data-color-token="--rdf-classexpr"],
  .ds-token-segment[data-color-token="--rdf-classexpr"] { --ds-token-color: var(--rdf-classexpr); }
  &[data-color-token="--rdf-nodeshape"],
  .ds-token-segment[data-color-token="--rdf-nodeshape"] { --ds-token-color: var(--rdf-nodeshape); }
  &[data-color-token="--rdf-propshape"],
  .ds-token-segment[data-color-token="--rdf-propshape"] { --ds-token-color: var(--rdf-propshape); }
  &[data-color-token="--rdf-resource"],
  .ds-token-segment[data-color-token="--rdf-resource"] { --ds-token-color: var(--rdf-resource); }
  &[data-color-token="--rdf-shacl"],
  .ds-token-segment[data-color-token="--rdf-shacl"] { --ds-token-color: var(--rdf-shacl); }

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

function clampPercent(value: number) {
  return Math.max(0, Math.min(100, Number.isFinite(value) ? value : 0));
}

export function TokenSwatch({ colorToken, className = "", ...props }: TokenSwatchProps) {
  return (
    <span
      className={cx("ds-token-swatch", tokenSwatchUX, colorTokenSkinX, className)}
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
          strokeDasharray={`${clamped} ${100 - clamped}`}
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
              strokeDasharray={`${percent} ${100 - percent}`}
              strokeDashoffset={dashOffset}
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
