import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		include: ["integration/**/*.test.ts"],
		// Refuses to run (or re-bless — see update-goldens.ts) against a
		// server that is not serving the repository's polyfill.toml.
		globalSetup: ["./integration/preflight.ts"],
		globals: true,
		environment: "node",
		testTimeout: 60_000,
		// The goldens' beforeAll fetches two bundles; the largest take longer
		// than vitest's 10s default to build on an unoptimized (CI) binary.
		hookTimeout: 60_000,
	},
});
