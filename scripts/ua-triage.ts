// UA parser triage harness (TODO item 5).
//
// Runs a broad set of user agents — this repo's fixtures, the pzb corpus,
// and intoli's daily-updated real-traffic dataset — through our Rust parser
// and through the recovered upstream normaliser
// (@financial-times/polyfill-useragent-normaliser@1.10.2, executed under
// node), then reports:
//
//   1. disagreements with upstream that are not documented intentional
//      divergences (candidate transliteration bugs), and
//   2. the heaviest real-traffic UAs we classify as unknown (candidate
//      missing browser rules).
//
// Usage (from the repo root, with mise's node and a rust toolchain):
//
//   node scripts/ua-triage.ts
//
// Each resolved finding should become a row in
// library/tests/fixtures/ua_table.json or an entry in the normaliser
// corpus fixture's intentional_divergences.

import { execFileSync, execSync } from "node:child_process";
import { createRequire } from "node:module";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { gunzipSync } from "node:zlib";

const repo = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");
const cache = path.join(repo, "scripts", ".cache");
mkdirSync(cache, { recursive: true });

const NORMALISER = "@financial-times/polyfill-useragent-normaliser";
const PZB_URL =
	"https://gist.githubusercontent.com/pzb/b4b6f57144aea7827ae4/raw/cf847b76a142955b1410c8bcef3aabe221a63db1/user-agents.txt";
const INTOLI_URL =
	"https://raw.githubusercontent.com/intoli/user-agents/main/src/user-agents.json.gz";

async function download(url: string, file: string, always = false): Promise<Buffer> {
	const target = path.join(cache, file);
	if (always || !existsSync(target)) {
		console.log(`fetching ${url}`);
		const response = await fetch(url);
		if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`);
		writeFileSync(target, Buffer.from(await response.arrayBuffer()));
	}
	return readFileSync(target);
}

// --- assemble inputs -------------------------------------------------------

interface FixtureCase {
	ua: string;
}

const uaTable: FixtureCase[] = JSON.parse(
	readFileSync(path.join(repo, "library/tests/fixtures/ua_table.json"), "utf8")
);
const ftParser = JSON.parse(
	readFileSync(path.join(repo, "library/tests/fixtures/ft_useragent_corpus.json"), "utf8")
);
const ftNormaliser = JSON.parse(
	readFileSync(path.join(repo, "library/tests/fixtures/ft_normaliser_corpus.json"), "utf8")
);
const intentional = new Map<string, string>(
	ftNormaliser.intentional_divergences.map((d: { ua: string; reason: string }) => [d.ua, d.reason])
);

// This fork's iOS patch-version fix (commit 5137b7996) is a *pattern*
// divergence: upstream's two-component regex sends any "OS x_y_z" UA to the
// WebKit fallback table (ios_saf/11.0.0); we return the real OS version.
// Current iOS in-app webview traffic (Google app, Twitter, …) hits this
// constantly, so it is exempted as a class rather than per-UA.
function isKnownDivergencePattern(ua: string, upstream: string, ours: string): boolean {
	return (
		upstream === "ios_saf/11.0.0" &&
		ours.startsWith("ios_saf/") &&
		/(iPod|iPhone|iPad).+OS \d+_\d+_\d+ like Mac OS X/.test(ua)
	);
}

const pzb = (await download(PZB_URL, "pzb.txt")).toString("utf8").split("\n").filter(Boolean);
// intoli updates daily — always refetch so the triage tracks current traffic.
const intoli: { userAgent: string; weight: number }[] = JSON.parse(
	gunzipSync(await download(INTOLI_URL, "intoli.json.gz", true)).toString("utf8")
);

const weights = new Map<string, number>();
for (const { userAgent, weight } of intoli) {
	weights.set(userAgent, (weights.get(userAgent) ?? 0) + weight);
}

const uas = [
	...new Set([
		...uaTable.map(c => c.ua),
		...ftParser.cases.map((c: FixtureCase) => c.ua),
		...ftNormaliser.cases.map((c: FixtureCase) => c.ua),
		...pzb,
		...intoli.map(e => e.userAgent),
	]),
];
console.log(`${uas.length} unique user agents`);

// --- run both implementations ----------------------------------------------

console.log("building parse_ua…");
execSync("cargo build --release --example parse_ua", { cwd: repo, stdio: "inherit" });
const ours = execFileSync(path.join(repo, "target/release/examples/parse_ua"), ["--stdin-normalized"], {
	input: uas.join("\n"),
	maxBuffer: 64 * 1024 * 1024,
})
	.toString("utf8")
	.split("\n");

if (!existsSync(path.join(cache, "node_modules", NORMALISER))) {
	console.log("installing recovered upstream normaliser from npm…");
	execSync(`npm install --prefix ${JSON.stringify(cache)} --no-save ${NORMALISER}@1.10.2`, {
		stdio: "inherit",
	});
}
const require = createRequire(import.meta.url);
const UA = require(path.join(cache, "node_modules", NORMALISER));

// --- report -----------------------------------------------------------------

let disagreements = 0;
console.log("\n== disagreements with upstream 1.10.2 (excluding documented divergences):");
uas.forEach((ua, i) => {
	let upstream: string;
	try {
		upstream = UA.normalize(ua);
	} catch {
		upstream = "ERROR";
	}
	if (upstream !== ours[i] && !intentional.has(ua) && !isKnownDivergencePattern(ua, upstream, ours[i])) {
		disagreements++;
		console.log(`  upstream=${upstream}  ours=${ours[i]}\n    ${ua}`);
	}
});
if (disagreements === 0) console.log("  none");

const totalWeight = [...weights.values()].reduce((a, b) => a + b, 0);
const unknown: [string, number][] = [];
uas.forEach((ua, i) => {
	if (ours[i].startsWith("other/") && weights.has(ua)) {
		unknown.push([ua, weights.get(ua)!]);
	}
});
unknown.sort((a, b) => b[1] - a[1]);
const unknownWeight = unknown.reduce((a, [, w]) => a + w, 0);

console.log(
	`\n== unknown-classified share of intoli real-traffic weight: ${((100 * unknownWeight) / totalWeight).toFixed(3)}%`
);
console.log("heaviest unknown-classified UAs:");
for (const [ua, weight] of unknown.slice(0, 10)) {
	console.log(`  ${((100 * weight) / totalWeight).toFixed(4)}%  ${ua}`);
}

console.log(
	`\ndone: ${disagreements} undocumented disagreement(s). Resolve each into a ua_table row, a fix, or a documented divergence.`
);
process.exitCode = disagreements > 0 ? 1 : 0;
