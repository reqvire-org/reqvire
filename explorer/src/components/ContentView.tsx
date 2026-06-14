import { css, cx } from "@linaria/atomic";
import { useStore } from "../store/StoreContext";
import { MarkdownContent } from "./MarkdownContent";
import { routeForView } from "../router/routes";
import { SourceCodePreview } from "./SourceCodePreview";

const routeBaseUX = css`
  position: relative;
  display: grid;
  box-sizing: border-box;
  grid-template-columns: minmax(0, 1fr) !important;
  column-gap: 0;
  height: 100vh;
  min-height: 0;
  padding-right: 0;
  padding-left: var(--ex-current-left-width);

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: hidden;
    padding-right: 0;
    padding-left: 0;
  }
`;

const routeSkinX = css`
  background: var(--bg-canvas);
  color: var(--text-body);

  .ex-app & {
    background: var(--bg-canvas);
  }
`;

const documentPanelBaseUX = css`
  position: relative;
  box-sizing: border-box;
  min-width: 0;
  min-height: 0;
  overflow: auto;
  padding: var(--space-14) var(--space-16);

  .ex-app & {
    height: 100%;
    min-height: 0;
    overflow: auto;
    padding: var(--space-16);
  }
`;

const documentPanelSkinX = css`
  border-right: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  border-left: var(--border-w) solid color-mix(in srgb, var(--border-subtle) 65%, transparent);
  background: var(--bg-surface);

  .ex-app & {
    border-right: 0;
    border-left: 0;
    background: var(--bg-surface);
  }
`;

const emptyBaseUX = css`
  font-size: var(--text-sm);
  font-style: italic;
  line-height: 1.45;
`;

const emptySkinX = css`
  color: var(--text-muted);
`;

const toolbarBaseUX = css`
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-6);
  margin-bottom: var(--space-8);
  padding: 0 0 var(--space-6);

  .ex-content-page__title {
    display: grid;
    min-width: 0;
    gap: var(--space-1);
  }

  .ex-content-page__title span {
    font-size: var(--text-micro);
    font-weight: var(--weight-bold);
    letter-spacing: 0.07em;
    text-transform: uppercase;
  }

  .ex-content-page__title strong {
    min-width: 0;
    overflow: hidden;
    font-size: var(--text-base);
    text-overflow: ellipsis;
    white-space: nowrap;
  }
`;

const toolbarSkinX = css`
  border-bottom: var(--border-w) solid var(--border-subtle);

  .ex-content-page__title span {
    color: var(--text-muted);
  }

  .ex-content-page__title strong {
    color: var(--text-body);
  }
`;

const commandBaseUX = css`
  border: 0;
  border-radius: var(--radius-md);
  padding: var(--space-3) var(--space-4);
  cursor: pointer;
  font-size: var(--text-sm);
  text-align: left;
`;

const commandSkinX = css`
  background: var(--accent);
  color: var(--accent-fg);
`;

interface ContentViewProps {
  path: string;
}

export function ContentView({ path }: ContentViewProps) {
  const { store } = useStore();
  const [filePath, fragmentId] = splitContentPath(path);
  const file = store.files.find((f) => f.path === filePath);
  const sourceResource = store.resources.find(
    (resource) => resource.file_path === filePath || resource.target === filePath,
  );
  const sourcePath = sourceResource?.file_path ?? sourceResource?.target ?? filePath;

  if (!file) {
    if (sourceResource?.source_text) {
      return (
        <div className={cx("ex-content-route", routeBaseUX, routeSkinX)}>
          <div className={cx("ex-content-document-panel", documentPanelBaseUX, documentPanelSkinX)}>
            <ContentToolbar filePath={sourcePath} label="Source file" />
            <SourceCodePreview
              path={sourcePath}
              content={sourceResource.source_text}
              kind={sourceResource.kind}
              relationTypes={sourceResource.relation_types}
              showPath={false}
            />
          </div>
        </div>
      );
    }

    return (
      <div className={cx("ex-content-route", routeBaseUX, routeSkinX)}>
        <div className={cx("ex-content-document-panel", documentPanelBaseUX, documentPanelSkinX)}>
          <ContentToolbar filePath={filePath} />
          <div className={cx("ex-empty", emptyBaseUX, emptySkinX)}>File not found: {filePath}</div>
        </div>
      </div>
    );
  }

  if (file.element_ids.length === 0 || sourceResource?.source_text) {
    return (
      <div className={cx("ex-content-route", routeBaseUX, routeSkinX)}>
        <div className={cx("ex-content-document-panel", documentPanelBaseUX, documentPanelSkinX)}>
          <ContentToolbar filePath={file.path} label="Source file" />
          <SourceCodePreview
            path={file.path}
            content={sourceResource?.source_text ?? file.markdown_content}
            kind={sourceResource?.kind ?? "source file"}
            relationTypes={sourceResource?.relation_types ?? []}
            showPath={false}
          />
        </div>
      </div>
    );
  }

  return (
    <div className={cx("ex-content-route", routeBaseUX, routeSkinX)}>
      <div className={cx("ex-content-document-panel", documentPanelBaseUX, documentPanelSkinX)}>
        <ContentToolbar filePath={file.path} />
        <MarkdownContent
          markdown={file.markdown_content}
          sourceFilePath={file.path}
          sourceAnchor={fragmentId ? `#/content/${file.path}#${fragmentId}` : `#/content/${file.path}`}
          scrollToAnchor={fragmentId}
        />
      </div>
    </div>
  );
}

function ContentToolbar({ filePath, label = "Source page" }: { filePath: string; label?: string }) {
  return (
    <div className={cx("ex-content-page__toolbar", toolbarBaseUX, toolbarSkinX)}>
      <div className={cx("ex-content-page__title")}>
        <span>{label}</span>
        <strong>{filePath || "Unknown file"}</strong>
      </div>
      <a className={cx("ex-content-page__command", commandBaseUX, commandSkinX)} href={routeForView("model")}>
        Back to model
      </a>
    </div>
  );
}

function splitContentPath(path: string) {
  const fragmentIndex = path.indexOf("#");
  if (fragmentIndex === -1) return [path, null] as const;
  return [path.slice(0, fragmentIndex), path.slice(fragmentIndex + 1) || null] as const;
}
