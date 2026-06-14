import type { ImgHTMLAttributes } from "react";
import { css, cx } from "@linaria/atomic";
import logoMarkUrl from "../../assets/logo-mark.svg?url";

export interface BrandMarkProps extends Omit<ImgHTMLAttributes<HTMLImageElement>, "src"> {
  decorative?: boolean;
}

const baseUX = css`
  display: block;
  width: var(--rq-brandmark-size, var(--icon-lg));
  height: var(--rq-brandmark-size, var(--icon-lg));
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
      className={cx("rq-brandmark", baseUX, className)}
      alt={decorative ? "" : alt}
      aria-hidden={decorative || undefined}
      {...props}
    />
  );
}
