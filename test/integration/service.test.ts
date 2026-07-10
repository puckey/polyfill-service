import { describe, expect, it } from "vitest";
import { request } from "./helpers.ts";

// These tests expect a server running with the repository's sample
// polyfill.toml: version 5.3.1, features = ["default", "fetch"].
const OLD_CHROME =
	"Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.22 (KHTML, like Gecko) Chrome/25.0.1364.172 Safari/537.22";
const MODERN_CHROME =
	"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36";
const IE_11 = "Mozilla/5.0 (Windows NT 6.1; Trident/7.0; rv:11.0) like Gecko";

describe("GET /", () => {
	it("responds with the info page", async () => {
		const response = await request("/");
		expect(response.status).toBe(200);
		expect(response.headers.get("content-type")).toMatch(/text\/html/);
		expect(response.body).toContain("polyfill service");
		expect(response.body).toContain("5.3.1");
	});
});

describe("GET /robots.txt", () => {
	it("responds with a 200", async () => {
		const response = await request("/robots.txt");
		expect(response.status).toBe(200);
	});
});

describe("GET /not-a-route", () => {
	it("responds with a cacheable 404", async () => {
		const response = await request("/not-a-route");
		expect(response.status).toBe(404);
		expect(response.headers.get("cache-control")).toContain("immutable");
	});
});

describe("method handling", () => {
	it("OPTIONS responds with allowed methods", async () => {
		const response = await request("/polyfill.min.js", { method: "OPTIONS" });
		expect(response.status).toBe(200);
		expect(response.headers.get("allow")).toBe("OPTIONS, GET, HEAD");
	});

	it("POST responds with a 405", async () => {
		const response = await request("/polyfill.min.js", { method: "POST" });
		expect(response.status).toBe(405);
		expect(response.headers.get("allow")).toBe("GET, HEAD");
	});

	it("HEAD responds with headers and no body", async () => {
		const response = await request("/polyfill.min.js", {
			method: "HEAD",
			userAgent: IE_11,
		});
		expect(response.status).toBe(200);
		expect(response.headers.get("content-type")).toMatch(/text\/javascript/);
		expect(response.body).toBe("");
	});
});

describe("GET /polyfill.min.js", () => {
	it("serves polyfills to an old browser", async () => {
		const response = await request("/polyfill.min.js", { userAgent: OLD_CHROME });
		expect(response.status).toBe(200);
		expect(response.headers.get("content-type")).toMatch(
			/text\/javascript; charset=(utf|UTF)-8/
		);
		expect(response.headers.get("cache-control")).toBe(
			"public, s-maxage=31536000, max-age=604800, stale-while-revalidate=604800, stale-if-error=604800, immutable"
		);
		expect(response.headers.get("vary")).toContain("User-Agent");
		expect(response.headers.get("x-polyfill-version")).toBe("5.3.1");
		expect(response.body).toContain("(function(self, undefined)");
		// Chrome 25 predates Promise (<32) and fetch (<42)
		expect(response.body).toContain("Promise");
		expect(response.body).toContain("fetch");
	});

	it("serves an empty bundle to a current browser", async () => {
		const response = await request("/polyfill.min.js", { userAgent: MODERN_CHROME });
		expect(response.status).toBe(200);
		expect(response.body).not.toContain("(function(self, undefined)");
	});

	it("ignores query parameters", async () => {
		const plain = await request("/polyfill.min.js", { userAgent: OLD_CHROME });
		const withParams = await request(
			"/polyfill.min.js?features=Symbol&version=3.111.0&unknown=ignore&callback=evil",
			{ userAgent: OLD_CHROME }
		);
		expect(withParams.status).toBe(200);
		expect(withParams.body).toBe(plain.body);
	});

	it("serves the same bundle on the /v3 alias", async () => {
		const canonical = await request("/polyfill.min.js", { userAgent: OLD_CHROME });
		const alias = await request("/v3/polyfill.min.js", { userAgent: OLD_CHROME });
		expect(alias.body).toBe(canonical.body);
	});
});

describe("GET /polyfill.js", () => {
	it("serves a readable bundle with an explainer comment", async () => {
		const response = await request("/polyfill.js", { userAgent: OLD_CHROME });
		expect(response.status).toBe(200);
		expect(response.body).toContain("Polyfill service v5.3.1");
		expect(response.body).toContain("Features requested:");
	});
});
