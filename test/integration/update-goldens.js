// Re-blesses integration/goldens.json against a running server.
//
//   PORT=7676 ./target/release/polyfill-service &   # with the repo's polyfill.toml
//   cd test && node integration/update-goldens.js
//
// Review the diff before committing — every changed row must be explainable
// (config change, library version bump, or an intentional behavior change).

import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";
import axios from "./helpers.js";
import { extractFeatures } from "./golden-helpers.js";

const goldensPath = path.join(path.dirname(fileURLToPath(import.meta.url)), "goldens.json");
const goldens = JSON.parse(readFileSync(goldensPath, "utf8"));

for (const entry of goldens) {
	const min = await axios.get("/polyfill.min.js", {
		headers: { "User-Agent": entry.ua },
	});
	const raw = await axios.get("/polyfill.js", {
		headers: { "User-Agent": entry.ua },
	});
	if (min.status !== 200 || raw.status !== 200) {
		throw new Error(`${entry.name}: unexpected status ${min.status}/${raw.status}`);
	}

	entry.features = extractFeatures(raw.data);
	entry.minBytes = Buffer.byteLength(min.data);
	entry.minSha256 = createHash("sha256").update(min.data).digest("hex");
	console.log(`${entry.name}: ${entry.features.length} features, ${entry.minBytes} bytes`);
}

writeFileSync(goldensPath, JSON.stringify(goldens, null, "\t") + "\n");
console.log(`\nre-blessed ${goldens.length} goldens — review the diff before committing`);
