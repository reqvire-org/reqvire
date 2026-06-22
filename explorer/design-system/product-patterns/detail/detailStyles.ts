import { css } from "@linaria/atomic";

export const detailDialogBaseUX = css`
  --ux-detail-dialog-w: 1120px;
  --ux-detail-dialog-max-h: 980px;
  --ux-detail-dialog-body-max-h: 780px;
  --ux-detail-dialog-chrome-h: 176px;
  --ux-detail-chip-link-max-w: 100%;
  width: min(var(--ux-detail-dialog-w), calc(100vw - var(--space-24)));
  max-width: min(var(--ux-detail-dialog-w), calc(100vw - var(--space-24)));
  max-height: min(92vh, var(--ux-detail-dialog-max-h));

  @media (max-width: 720px) {
    width: calc(100vw - var(--space-10));
    max-width: calc(100vw - var(--space-10));
    max-height: calc(100vh - var(--space-10));
  }
`;

export const detailDialogSkinX = css`
  border: var(--border-w) solid var(--border-default);
  border-radius: var(--radius-xl);
  background: var(--bg-overlay);
  color: var(--text-body);
  box-shadow: var(--shadow-xl);

  .ux-markdown pre,
  .source-code-preview-body {
    border: var(--border-w) solid var(--border-subtle);
    background: var(--bg-sunken);
  }

  .ux-markdown code {
    background: var(--bg-sunken);
  }

  .ux-markdown h1,
  .ux-markdown h2,
  .ux-markdown h3,
  .ux-markdown h4 {
    margin: 0.85em 0 0.4em;
    color: var(--text-body);
    font-size: var(--text-sm);
    font-weight: var(--weight-bold);
    letter-spacing: 0.01em;
    line-height: 1.35;
  }
`;

export const detailHeaderBaseUX = css`
  display: grid;
  gap: var(--space-2);
  padding: var(--space-10) var(--space-10) var(--space-10) var(--space-14);

  @media (max-width: 720px) {
    padding: var(--space-8) var(--space-24) var(--space-8) var(--space-8);
  }
`;

export const detailHeaderSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);
  background: var(--bg-overlay);
`;

export const detailBodyBaseUX = css`
  max-height: min(74vh, var(--ux-detail-dialog-body-max-h));
  overflow: auto;
  padding: var(--space-14) var(--space-16);

  @media (max-width: 720px) {
    max-height: calc(100vh - var(--ux-detail-dialog-chrome-h));
    padding: var(--space-8);
  }
`;

export const detailBodySkinX = css`
  background: var(--bg-surface);
`;

export const detailTitleRowUX = css`
  display: flex;
  width: 100%;
  min-width: 0;
  align-items: center;
  gap: var(--space-2);

  h2 {
    flex: 1 1 auto;
    min-width: 0;
  }
`;

export const detailBackUX = css`
  width: var(--space-10);
  height: var(--space-10);
  flex: 0 0 auto;
  border-radius: var(--radius-sm);
`;

export const detailFamilyBadgeUX = css`
  flex: 0 0 auto;
`;

export const detailCloseUX = css`
  flex: 0 0 auto;
  margin-left: auto;
  margin-right: calc(-1 * var(--space-3));

  svg {
    width: var(--icon-sm);
    height: var(--icon-sm);
  }
`;

export const detailContentFlowUX = css`
  display: flex;
  width: 100%;
  flex-direction: column;
  gap: var(--space-14);
`;

export const detailSectionUX = css`
  display: grid;
  gap: var(--space-4);

  h3 {
    margin: 0;
    color: var(--text-strong);
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
    letter-spacing: 0;
    line-height: var(--leading-tight);
  }
`;

export const detailMutedUX = css`
  margin: 0;
  color: var(--text-muted);
  font-size: var(--text-sm);
`;

export const governanceListUX = css`
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3) var(--space-6);
`;

export const governanceItemBaseUX = css`
  display: inline-flex;
  align-items: baseline;
  gap: var(--space-3);
  padding: 0 var(--space-2);
  font-size: var(--text-sm);
`;

export const governanceItemSkinX = css`
  border-radius: var(--radius-sm);
  background: transparent;
`;

export const governanceKeySkinX = css`
  color: var(--text-muted);
`;

export const governanceValueSkinX = css`
  color: var(--text-body);
  font-weight: var(--weight-regular);
`;

export const governanceProvenanceSkinX = css`
  border-radius: var(--radius-pill);
  background: transparent;
  padding: 0;
  color: var(--text-faint);
  font-size: var(--text-sm);
`;

export const relationStackUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--stack-gap-compact);
`;

export const relationListUX = css`
  display: grid;
  grid-template-columns: max-content minmax(0, 1fr);
  align-items: center;
  column-gap: var(--space-5);
  gap: var(--stack-gap-compact);
`;

export const relationRowBaseUX = css`
  display: contents;
`;

export const conceptReferenceRowBaseUX = css`
  display: flex;
  align-items: center;
  gap: var(--space-3);
  min-width: 0;
  padding: var(--space-2) 0;
`;

export const conceptReferenceRowSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);

  &:last-child {
    border-bottom: 0;
  }
`;

export const conceptReferenceQualifierSkinX = css`
  min-width: 0;
  color: var(--text-muted);
  font-size: var(--text-sm);
  overflow-wrap: anywhere;
`;

export const relationTextSkinX = css`
  color: var(--text-body);
  font-size: var(--text-sm);
`;

export const relationKindBaseUX = css`
  display: inline-flex;
  align-items: center;
  justify-self: start;
  gap: var(--space-3);
  padding: var(--space-1) var(--space-5);
  font-family: var(--font-mono);
  font-size: var(--text-caption);
  line-height: 1.45;
`;

export const relationKindSkinX = css`
  border-radius: var(--radius-pill);
  background: var(--bg-sunken);
  color: var(--text-muted);
`;

export const relationEndpointBaseUX = css`
  display: inline-flex;
  justify-self: start;
  min-width: 0;
  max-width: min(100%, var(--ux-detail-chip-link-max-w));
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-1) var(--space-3);
  border: 0;
  font-family: inherit;
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  line-height: 1.45;
  text-decoration: none;
  overflow-wrap: anywhere;

  &:is(a, button) {
    cursor: pointer;
  }
`;

export const relationEndpointSkinX = css`
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-strong);

  &:hover {
    background: var(--bg-hover);
    text-decoration: none;
  }
`;

export const relationEndpointLabelUX = css`
  min-width: 0;
  overflow-wrap: anywhere;
`;

export const detailFooterBaseUX = css`
  padding: var(--space-7) var(--space-16);

  @media (max-width: 720px) {
    padding: var(--space-6) var(--space-8);
  }
`;

export const detailFooterSkinX = css`
  border-top: var(--border-w) solid var(--border-subtle);
  background: var(--bg-overlay);
`;

export const detailFooterRowUX = css`
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-8);
`;

export const sourceLinkBaseUX = css`
  display: inline-flex;
  align-items: center;
  gap: var(--space-3);
  min-width: 0;
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  text-decoration: none;
  overflow-wrap: anywhere;
`;

export const sourceLinkSkinX = css`
  color: var(--accent);

  &:hover {
    text-decoration: underline;
    text-underline-offset: var(--space-1);
  }
`;

export const iconSmUX = css`
  width: var(--space-8);
  height: var(--space-8);
  flex: none;
`;
