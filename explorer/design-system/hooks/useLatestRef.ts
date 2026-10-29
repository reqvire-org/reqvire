import { useLayoutEffect, useRef } from "preact/hooks";
import { type RefObject } from "preact";

export function useLatestRef<T>(value: T): RefObject<T> {
  const ref = useRef(value);

  useLayoutEffect(() => {
    ref.current = value;
  }, [value]);

  return ref;
}
