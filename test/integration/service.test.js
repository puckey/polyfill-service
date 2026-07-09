/* eslint-env mocha */

"use strict";

import assert from "node:assert";
import axios from "./helpers.js";

// These tests expect a server running with the repository's sample
// polyfill.toml: version 5.3.1, features = ["default", "fetch"].
const OLD_CHROME =
	"Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.22 (KHTML, like Gecko) Chrome/25.0.1364.172 Safari/537.22";
const MODERN_CHROME =
	"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36";

describe("GET /", function () {
	it("responds with the info page", async () => {
		const response = await axios.get("/");
		assert.equal(response.status, 200);
		assert.match(response.headers["content-type"], /text\/html/);
		assert.ok(response.data.includes("polyfill service"));
		assert.ok(response.data.includes("5.3.1"));
	});
});

describe("GET /robots.txt", function () {
	it("responds with a 200", async () => {
		const response = await axios.get("/robots.txt");
		assert.equal(response.status, 200);
	});
});

describe("GET /not-a-route", function () {
	it("responds with a cacheable 404", async () => {
		const response = await axios.get("/not-a-route");
		assert.equal(response.status, 404);
		assert.ok(response.headers["cache-control"].includes("immutable"));
	});
});

describe("method handling", function () {
	it("OPTIONS responds with allowed methods", async () => {
		const response = await axios.options("/polyfill.min.js");
		assert.equal(response.status, 200);
		assert.equal(response.headers["allow"], "OPTIONS, GET, HEAD");
	});

	it("POST responds with a 405", async () => {
		const response = await axios.post("/polyfill.min.js", {});
		assert.equal(response.status, 405);
		assert.equal(response.headers["allow"], "GET, HEAD");
	});
});

describe("GET /polyfill.min.js", function () {
	it("serves polyfills to an old browser", async () => {
		const response = await axios.get("/polyfill.min.js", {
			headers: { "User-Agent": OLD_CHROME },
		});
		assert.equal(response.status, 200);
		assert.match(response.headers["content-type"], /text\/javascript; charset=(utf|UTF)-8/);
		assert.equal(
			response.headers["cache-control"],
			"public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable"
		);
		assert.ok(response.headers["vary"].includes("User-Agent"));
		assert.equal(response.headers["x-polyfill-version"], "5.3.1");
		assert.ok(response.data.includes("(function(self, undefined)"));
		// Chrome 25 predates Promise (<32) and fetch (<42)
		assert.ok(response.data.includes("Promise"));
		assert.ok(response.data.includes("fetch"));
	});

	it("serves an empty bundle to a current browser", async () => {
		const response = await axios.get("/polyfill.min.js", {
			headers: { "User-Agent": MODERN_CHROME },
		});
		assert.equal(response.status, 200);
		assert.ok(!response.data.includes("(function(self, undefined)"));
	});

	it("ignores query parameters", async () => {
		const plain = await axios.get("/polyfill.min.js", {
			headers: { "User-Agent": OLD_CHROME },
		});
		const withParams = await axios.get(
			"/polyfill.min.js?features=Symbol&version=3.111.0&unknown=ignore&callback=evil",
			{ headers: { "User-Agent": OLD_CHROME } }
		);
		assert.equal(withParams.status, 200);
		assert.equal(withParams.data, plain.data);
	});

	it("serves the same bundle on the /v3 alias", async () => {
		const canonical = await axios.get("/polyfill.min.js", {
			headers: { "User-Agent": OLD_CHROME },
		});
		const alias = await axios.get("/v3/polyfill.min.js", {
			headers: { "User-Agent": OLD_CHROME },
		});
		assert.equal(alias.data, canonical.data);
	});
});

describe("GET /polyfill.js", function () {
	it("serves a readable bundle with an explainer comment", async () => {
		const response = await axios.get("/polyfill.js", {
			headers: { "User-Agent": OLD_CHROME },
		});
		assert.equal(response.status, 200);
		assert.ok(response.data.includes("Polyfill service v5.3.1"));
		assert.ok(response.data.includes("Features requested:"));
	});
});
