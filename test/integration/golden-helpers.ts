import { baseURL, request } from "./helpers.ts";

export interface GoldenEntry {
	name: string;
	ua: string;
	/** Skip jsdom execution; `note` must say why. */
	parseOnly?: boolean;
	note?: string;
	features: string[];
	minSha256: string;
	minBytes: number;
}

// The readable bundle lists every included polyfill in its explainer
// comment as " * - <Name>, License: ..." lines. That list is the resolved
// feature set for the requesting UA.
export function extractFeatures(rawBundle: string): string[] {
	const comment = rawBundle.split("*/")[0];
	return [...comment.matchAll(/^ \* - ([^,]+), License/gm)].map(match => match[1]);
}

// Guard for both the test run and the bless script: whatever answers on the
// test port must be this service running the repository's sample
// polyfill.toml, otherwise golden comparisons — and worse, re-blessing —
// are meaningless. The markers are the exact structural fragments the info
// page renders for this config, not incidental substrings.
export async function assertServerMatchesRepoConfig(): Promise<void> {
	let response;
	try {
		response = await request("/");
	} catch {
		throw new Error(
			`no server responding at ${baseURL} — start one with the repo's polyfill.toml (PORT=7676 ./target/release/polyfill-service)`
		);
	}
	const fingerprint = [
		"<title>polyfill service</title>",
		"polyfill-library 5.3.1",
		"<li><code>default</code></li>",
		"<li><code>fetch</code></li>",
	];
	for (const marker of fingerprint) {
		if (response.status !== 200 || !response.body.includes(marker)) {
			throw new Error(
				`server at ${baseURL} is not running the repo's polyfill.toml (missing ${JSON.stringify(marker)}) — refusing to compare or re-bless goldens against it`
			);
		}
	}
}
