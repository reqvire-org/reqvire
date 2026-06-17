import { StoreNotice, type StoreNoticeProps } from "@ds";

export type MissingStoreNoticeProps = StoreNoticeProps;

export function MissingStoreNotice(props: MissingStoreNoticeProps) {
  return <StoreNotice {...props} />;
}
