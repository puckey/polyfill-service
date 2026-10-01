import { readFileSync } from "node:fs";
import { createContext, runInContext } from "node:vm";
import { describe, expect, it } from "vitest";

const LIBRARY = new URL("../../polyfill-libraries/5.3.1/", import.meta.url);

// Error.cause and what it depends on, in dependency order, as the bundle
// would order them.
const SOURCES = [
	"_ESAbstract.DefinePropertyOrThrow",
	"_ESAbstract.CreateMethodProperty",
	"_ESAbstract.CreateNonEnumerableDataPropertyOrThrow",
	"_ESAbstract.Get",
	"_ESAbstract.HasProperty",
	"_ESAbstract.Type",
	"_ErrorConstructor",
	"Error.cause",
];

/** A fresh global with the polyfill installed, like a browser without Error.cause. */
function polyfilled(): { run(code: string): unknown } {
	const context = createContext({});
	runInContext("var self = this;", context);
	for (const name of SOURCES) {
		runInContext(readFileSync(new URL(`${name}/raw.js`, LIBRARY), "utf8"), context, {
			filename: `${name}/raw.js`,
		});
	}
	return { run: (code) => runInContext(code, context) };
}

describe("Error.cause", () => {
	it("replaces the native constructors", () => {
		const { run } = polyfilled();
		expect(run("Error.toString()")).not.toContain("[native code]");
		expect(run("TypeError.toString()")).not.toContain("[native code]");
	});

	it("installs cause", () => {
		const { run } = polyfilled();
		expect(run("new Error('m', { cause: 'c' }).cause")).toBe("c");
		expect(run("new TypeError('m', { cause: 'c' }).cause")).toBe("c");
		expect(run("'cause' in new Error('m')")).toBe(false);
	});

	it("keeps a direct Error an Error", () => {
		const { run } = polyfilled();
		expect(run("new Error('m') instanceof Error")).toBe(true);
		expect(run("Error('m') instanceof Error")).toBe(true);
		expect(run("new Error('m').constructor === Error")).toBe(true);
		expect(run("new RangeError('m') instanceof RangeError")).toBe(true);
		expect(run("new RangeError('m') instanceof Error")).toBe(true);
		expect(run("Object.prototype.toString.call(new Error('m'))")).toBe("[object Error]");
	});

	describe("a class extending it", () => {
		const SUBCLASS = `
			class LoadError extends Error {
				constructor(key) {
					super('did not load', { cause: 'offline' });
					this.name = 'LoadError';
					this.key = key;
				}
			}
			class RangeSubclass extends RangeError {}
		`;

		it("is an instance of itself", () => {
			const { run } = polyfilled();
			run(SUBCLASS);
			expect(run("new LoadError('2/2/2') instanceof LoadError")).toBe(true);
			expect(run("new LoadError('2/2/2') instanceof Error")).toBe(true);
			expect(run("new RangeSubclass('m') instanceof RangeSubclass")).toBe(true);
			expect(run("new RangeSubclass('m') instanceof RangeError")).toBe(true);
		});

		it("keeps its constructor, its own properties and its cause", () => {
			const { run } = polyfilled();
			run(SUBCLASS);
			expect(run("new LoadError('2/2/2').constructor === LoadError")).toBe(true);
			expect(run("Object.hasOwn(new LoadError('2/2/2'), 'constructor')")).toBe(false);
			expect(run("new LoadError('2/2/2').key")).toBe("2/2/2");
			expect(run("new LoadError('2/2/2').name")).toBe("LoadError");
			expect(run("new LoadError('2/2/2').message")).toBe("did not load");
			expect(run("new LoadError('2/2/2').cause")).toBe("offline");
			expect(run("typeof new LoadError('2/2/2').stack")).toBe("string");
		});
	});
});
