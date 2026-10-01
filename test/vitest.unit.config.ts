import { defineConfig } from "vitest/config";

// Unit tests over the vendored polyfill sources themselves; no server needed.
export default defineConfig({
	test: {
		include: ["unit/**/*.test.ts"],
		globals: true,
		environment: "node",
	},
});
