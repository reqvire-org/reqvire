import { cx } from "@linaria/atomic";
import {
  governanceItemBaseUX,
  governanceItemSkinX,
  governanceKeySkinX,
  governanceListUX,
  governanceProvenanceSkinX,
  governanceValueSkinX,
} from "./detailStyles";
import type { DetailMetaBadge } from "./types";

export interface MetadataStripProps {
  badges: DetailMetaBadge[];
}

export function MetadataStrip({ badges }: MetadataStripProps) {
  if (badges.length === 0) return null;

  return (
    <div className={cx(governanceListUX)}>
      {badges.map(({ key, value, provenance }) => (
        <span className={cx(governanceItemBaseUX, governanceItemSkinX)} key={`meta-${key}`}>
          <span className={cx(governanceKeySkinX)}>{key}</span>
          <span className={cx(governanceValueSkinX)}>{value}</span>
          <span className={cx(governanceProvenanceSkinX)}>({provenance})</span>
        </span>
      ))}
    </div>
  );
}
