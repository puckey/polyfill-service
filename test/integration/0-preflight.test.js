/* eslint-env mocha */

"use strict";

import axios from "./helpers.js";
import { assertServerMatchesRepoConfig } from "./golden-helpers.js";

// Root-level hook: if the server on the test port is not this service
// running the repository's polyfill.toml, abort the entire run with this
// single error instead of burying the diagnosis under 100+ golden failures.
before("preflight: server matches the repo config", async () => {
	await assertServerMatchesRepoConfig(axios);
});
