/**
 * The namespace the UI operates in.
 *
 * This UUID used to be pasted literally into seven places, including as a silent fallback
 * in page components and as an override in useServerForm that discarded whatever the caller
 * passed. One definition means one thing to change when namespaces stop being a constant.
 *
 * NEXT_PUBLIC_* is inlined at build time, so this resolves at build, not at runtime. It must
 * be supplied as a build arg (see frontend/Dockerfile); passing it as a runtime environment
 * variable silently leaves the fallback in place.
 */
export const DEFAULT_NAMESPACE_ID = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";

export const NAMESPACE_ID =
  process.env.NEXT_PUBLIC_NAMESPACE_ID?.trim() || DEFAULT_NAMESPACE_ID;
