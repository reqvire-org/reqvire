import { useStore } from "../store/StoreContext";
import { MarkdownContent } from "./MarkdownContent";
import { routeForView } from "../router/routes";
import { SourceCodePreview } from "./SourceCodePreview";

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
        <div className="ex-route ex-route-single">
          <div className="ex-document-panel">
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
      <div className="ex-route ex-route-single">
        <div className="ex-document-panel">
          <ContentToolbar filePath={filePath} />
          <div className="ex-empty">File not found: {filePath}</div>
        </div>
      </div>
    );
  }

  if (file.element_ids.length === 0 || sourceResource?.source_text) {
    return (
      <div className="ex-route ex-route-single">
        <div className="ex-document-panel">
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
    <div className="ex-route ex-route-single">
      <div className="ex-document-panel">
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
    <div className="content-page-toolbar">
      <div className="content-page-title">
        <span>{label}</span>
        <strong>{filePath || "Unknown file"}</strong>
      </div>
      <a className="ex-command" href={routeForView("model")}>
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
