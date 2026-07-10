import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import vm from "node:vm";
import { JSDOM } from "jsdom";
import { beforeAll, describe, expect, it } from "vitest";
import { request } from "./helpers.ts";
import { extractFeatures, type GoldenEntry } from "./golden-helpers.ts";

// Golden-file suite: with a fixed polyfill.toml and store, the bundle is a
// pure function of the User-Agent string. These snapshots pin it. A failing
// hash means the bundle changed for that browser — explain the diff (the
// `features` list shows what moved), then re-bless:
//
//   cd test && npm run bless

const goldensPath = path.join(path.dirname(fileURLToPath(import.meta.url)), "goldens.json");
const goldens: GoldenEntry[] = JSON.parse(readFileSync(goldensPath, "utf8"));

describe("golden bundles", () => {
	for (const entry of goldens) {
		describe(entry.name, () => {
			let min: string;
			let raw: string;

			beforeAll(async () => {
				const minResponse = await request("/polyfill.min.js", { userAgent: entry.ua });
				const rawResponse = await request("/polyfill.js", { userAgent: entry.ua });
				expect(minResponse.status).toBe(200);
				expect(rawResponse.status).toBe(200);
				min = minResponse.body;
				raw = rawResponse.body;
			});

			it("serves the blessed feature set", () => {
				expect(extractFeatures(raw)).toEqual(entry.features);
			});

			it("serves the blessed minified bundle", () => {
				const sha = createHash("sha256").update(min).digest("hex");
				expect(
					sha,
					`bundle changed (${Buffer.byteLength(min)} bytes vs blessed ${entry.minBytes})`
				).toBe(entry.minSha256);
			});

			it("parses as JavaScript", () => {
				new vm.Script(min);
				new vm.Script(raw);
			});

			// Execution is the default; entries opt out with parseOnly (and
			// must say why in a note) so a forgotten flag fails loudly
			// instead of silently skipping.
			if (!entry.parseOnly) {
				it("executes and installs the expected globals", () => {
					const dom = new JSDOM("", { runScripts: "outside-only" });
					const context = dom.getInternalVMContext();
					new vm.Script(raw).runInContext(context);

					// jsdom has no fetch of its own, so if the bundle included
					// the fetch polyfill it must be installed now — and if it
					// did not (modern browsers), fetch must stay absent.
					if (entry.features.includes("fetch")) {
						expect(typeof dom.window.fetch, "fetch polyfill not installed").toBe("function");
					} else {
						expect(dom.window.fetch, "fetch appeared without being served").toBeUndefined();
					}
					if (entry.features.includes("Promise")) {
						expect(typeof dom.window.Promise, "Promise missing after bundle ran").toBe(
							"function"
						);
					}
				});
			}
		});
	}
});
