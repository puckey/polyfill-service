/* eslint-env mocha */

"use strict";

import assert from "node:assert";
import axios from "./helpers.js";

// Guard against running the suite (and especially re-blessing goldens)
// against the wrong server: whatever is on the port must be this service
// running the repository's sample polyfill.toml.
describe("preflight: server matches the repo config", function () {
	it("is the polyfill service with version 5.3.1 and features default, fetch", async () => {
		let response;
		try {
			response = await axios.get("/");
		} catch (error) {
			assert.fail(
				`no server responding at ${axios.defaults.baseURL} — start one with the repo's polyfill.toml (PORT=7676 ./target/release/polyfill-service)`
			);
		}
		assert.equal(response.status, 200);
		const fingerprint = ["polyfill service", "polyfill-library 5.3.1", "default", "fetch"];
		for (const marker of fingerprint) {
			assert.ok(
				response.data.includes(marker),
				`server at ${axios.defaults.baseURL} is not running the repo's polyfill.toml (missing "${marker}") — goldens would be meaningless against it`
			);
		}
	});
});
