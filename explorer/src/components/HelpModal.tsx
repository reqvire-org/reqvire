import { HelpDialog, type HelpDialogProps } from "@ds";

export type HelpModalProps = HelpDialogProps;

export function HelpModal(props: HelpModalProps) {
  return <HelpDialog {...props} />;
}
