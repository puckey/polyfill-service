// Re-blesses integration/goldens.json against a running server.
//
//   PORT=7676 ./target/release/polyfill-service &   # with the repo's polyfill.toml
//   cd test && pnpm bless
//
// Review the diff before committing — every changed row must be explainable
// (config change, library version bump, or an intentional behavior change).

import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { request } from "./helpers.ts";
import {
	assertServerMatchesRepoConfig,
	extractFeatures,
	type GoldenEntry,
} from "./golden-helpers.ts";

// Re-blessing against the wrong server would silently rewrite the fixture
// everything else trusts — refuse before touching anything.
await assertServerMatchesRepoConfig();

const goldensPath = path.join(path.dirname(fileURLToPath(import.meta.url)), "goldens.json");
const goldens: GoldenEntry[] = JSON.parse(readFileSync(goldensPath, "utf8"));

for (const entry of goldens) {
	const min = await request("/polyfill.min.js", { userAgent: entry.ua });
	const raw = await request("/polyfill.js", { userAgent: entry.ua });
	if (min.status !== 200 || raw.status !== 200) {
		throw new Error(`${entry.name}: unexpected status ${min.status}/${raw.status}`);
	}

	entry.features = extractFeatures(raw.body);
	entry.minBytes = Buffer.byteLength(min.body);
	entry.minSha256 = createHash("sha256").update(min.body).digest("hex");
	console.log(`${entry.name}: ${entry.features.length} features, ${entry.minBytes} bytes`);
}

writeFileSync(goldensPath, JSON.stringify(goldens, null, "\t") + "\n");
console.log(`\nre-blessed ${goldens.length} goldens — review the diff before committing`);
