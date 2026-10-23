import { type ComponentChildren } from "preact";
import { cx } from "@linaria/atomic";
import { detailSectionUX } from "./detailStyles";

export interface DetailSectionProps {
  title: string;
  children: ComponentChildren;
}

export function DetailSection({ title, children }: DetailSectionProps) {
  return (
    <section className={cx(detailSectionUX)}>
      <h3>{title}</h3>
      {children}
    </section>
  );
}
