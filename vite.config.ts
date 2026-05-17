import { defineConfig } from "vite-plus";

export default defineConfig({
  fmt: {},
  staged: {
    "*": "vp fmt --no-error-on-unmatched-pattern",
  },
});
