// Vitest global setup: runs once before any test file. Throwing here
// aborts the entire run with this single error instead of burying the
// diagnosis under a hundred golden-hash failures.
import { assertServerMatchesRepoConfig } from "./golden-helpers.ts";

export default async function preflight(): Promise<void> {
	await assertServerMatchesRepoConfig();
}
