import type { ReactNode } from "react";
import { css, cx } from "@linaria/atomic";
import { TypeBadge } from "../../components/data/TypeBadge";

export interface SourcePageElementsProps {
  children: ReactNode;
}

export interface SourcePageElementProps {
  id?: string;
  title: ReactNode;
  elementType?: string | null;
  typeFamily?: string | null;
  children: ReactNode;
}

const sourceElementsUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-16);
`;

const sourceElementUX = css`
  display: flex;
  flex-direction: column;
  gap: var(--space-8);
  padding-block: var(--space-7) var(--space-10);
  border-bottom: var(--border-w) solid var(--border-subtle);

  &:last-child {
    border-bottom: 0;
  }
`;

const sourceTitleRowUX = css`
  display: flex;
  min-width: 0;
  align-items: center;
  gap: var(--space-3);

  h2 {
    margin: 0;
    color: var(--text-strong);
    font-size: var(--text-xl);
    font-weight: var(--weight-bold);
    line-height: var(--leading-tight);
  }
`;

export function SourcePageElements({ children }: SourcePageElementsProps) {
  return <div className={cx(sourceElementsUX)}>{children}</div>;
}

export function SourcePageElement({ id, title, elementType, typeFamily, children }: SourcePageElementProps) {
  return (
    <article className={cx(sourceElementUX)} id={id}>
      <div className={cx(sourceTitleRowUX)}>
        {elementType ? (
          <TypeBadge type={elementType} family={typeFamily} tinted dot={false}>
            {elementType}
          </TypeBadge>
        ) : null}
        <h2>{title}</h2>
      </div>
      {children}
    </article>
  );
}
