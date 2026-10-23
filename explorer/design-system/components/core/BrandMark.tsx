import { type ImgHTMLAttributes } from "preact";
import { css, cx } from "@linaria/atomic";
import logoMarkUrl from "../../assets/logo-mark.svg?url";

export interface BrandMarkProps extends Omit<(ImgHTMLAttributes<HTMLImageElement> & { className?: string }), "src" | "style" | "alt" | "role"> {
  decorative?: boolean;
  alt?: string;
}

const baseUX = css`
  display: block;
  width: var(--ds-brandmark-size, var(--icon-lg));
  height: var(--ds-brandmark-size, var(--icon-lg));
  object-fit: contain;
`;

export function BrandMark({
  className = "",
  decorative = false,
  alt = "Reqvire",
  ...props
}: BrandMarkProps) {
  return (
    <img
      src={logoMarkUrl}
      className={cx("ds-brandmark", baseUX, className)}
      alt={decorative ? "" : alt}
      aria-hidden={decorative || undefined}
      {...props}
    />
  );
}
