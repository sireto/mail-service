import { defineConfig } from "eslint/config";
import nextCoreWebVitals from "eslint-config-next/core-web-vitals";
import nextTypescript from "eslint-config-next/typescript";

export default defineConfig([
  ...nextCoreWebVitals,
  ...nextTypescript,
  {
    rules: {
      // These were "off". Unused bindings and implicit `any` are exactly the checks that
      // catch a mistyped field or a dropped variable, so they are surfaced as warnings:
      // visible in review and in `yarn lint` output, without failing the build on the
      // existing backlog. Tighten to "error" once the backlog is cleared.
      "@typescript-eslint/no-unused-vars": [
        "warn",
        { argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
      ],
      "@typescript-eslint/no-explicit-any": "warn",
    },
  },
]);
