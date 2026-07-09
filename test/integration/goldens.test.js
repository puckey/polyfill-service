/* eslint-env mocha */

"use strict";

import assert from "node:assert";
import vm from "node:vm";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";
import { JSDOM } from "jsdom";
import axios from "./helpers.js";
import { extractFeatures } from "./golden-helpers.js";

// Golden-file suite: with a fixed polyfill.toml and store, the bundle is a
// pure function of the User-Agent string. These snapshots pin it. A failing
// hash means the bundle changed for that browser — explain the diff (the
// `features` list shows what moved), then re-bless:
//
//   cd test && node integration/update-goldens.js

const goldensPath = path.join(path.dirname(fileURLToPath(import.meta.url)), "goldens.json");
const goldens = JSON.parse(readFileSync(goldensPath, "utf8"));

describe("golden bundles", function () {
	for (const entry of goldens) {
		describe(entry.name, function () {
			let min;
			let raw;

			before(async () => {
				min = await axios.get("/polyfill.min.js", { headers: { "User-Agent": entry.ua } });
				raw = await axios.get("/polyfill.js", { headers: { "User-Agent": entry.ua } });
			});

			it("serves the blessed feature set", () => {
				assert.equal(raw.status, 200);
				assert.deepEqual(extractFeatures(raw.data), entry.features);
			});

			it("serves the blessed minified bundle", () => {
				assert.equal(min.status, 200);
				const sha = createHash("sha256").update(min.data).digest("hex");
				assert.equal(
					sha,
					entry.minSha256,
					`bundle changed (${Buffer.byteLength(min.data)} bytes vs blessed ${entry.minBytes})`
				);
			});

			it("parses as JavaScript", () => {
				new vm.Script(min.data);
				new vm.Script(raw.data);
			});

			if (entry.exec) {
				it("executes and installs the expected globals", () => {
					const dom = new JSDOM("", { runScripts: "outside-only" });
					const context = dom.getInternalVMContext();
					new vm.Script(raw.data).runInContext(context);

					// jsdom has no fetch of its own, so if the bundle included
					// the fetch polyfill it must be installed now — and if it
					// did not (modern browsers), fetch must stay absent.
					if (entry.features.includes("fetch")) {
						assert.equal(typeof dom.window.fetch, "function", "fetch polyfill not installed");
					} else {
						assert.equal(dom.window.fetch, undefined, "fetch appeared without being served");
					}
					if (entry.features.includes("Promise")) {
						assert.equal(typeof dom.window.Promise, "function", "Promise missing after bundle ran");
					}
				});
			}
		});
	}
});
