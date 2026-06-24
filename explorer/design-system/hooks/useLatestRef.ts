import { useLayoutEffect, useRef, type MutableRefObject } from "react";

export function useLatestRef<T>(value: T): MutableRefObject<T> {
  const ref = useRef(value);

  useLayoutEffect(() => {
    ref.current = value;
  }, [value]);

  return ref;
}
