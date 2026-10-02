import { readFileSync } from "node:fs";
import { createContext, runInContext } from "node:vm";
import { describe, expect, it } from "vitest";

const LIBRARY = new URL("../../polyfill-libraries/5.3.1/", import.meta.url);

// Array.prototype.at, String.prototype.at and what they depend on, in
// dependency order, as the bundle would order them.
const SOURCES = [
	"_ESAbstract.CreateMethodProperty",
	"_ESAbstract.Get",
	"_ESAbstract.Type",
	"_ESAbstract.ToInteger",
	"_ESAbstract.ToLength",
	"_ESAbstract.LengthOfArrayLike",
	"_ESAbstract.ToNumber",
	"_ESAbstract.ToIntegerOrInfinity",
	"_ESAbstract.ToObject",
	"_ESAbstract.Call",
	"_ESAbstract.GetV",
	"_ESAbstract.IsCallable",
	"_ESAbstract.GetMethod",
	"_ESAbstract.OrdinaryToPrimitive",
	"_ESAbstract.ToPrimitive",
	"_ESAbstract.ToString",
	"_ESAbstract.RequireObjectCoercible",
	"Array.prototype.at",
	"String.prototype.at",
];

/** A fresh global with the polyfills installed from `file`, like a browser without `at`. */
function polyfilled(file: "raw.js" | "min.js"): { run(code: string): unknown } {
	const context = createContext({});
	runInContext(
		"var self = this; delete Array.prototype.at; delete String.prototype.at;",
		context,
	);
	for (const name of SOURCES) {
		runInContext(readFileSync(new URL(`${name}/${file}`, LIBRARY), "utf8"), context, {
			filename: `${name}/${file}`,
		});
	}
	return { run: (code) => runInContext(code, context) };
}

describe.each(["raw.js", "min.js"] as const)("Array.prototype.at and String.prototype.at (%s)", (file) => {
	const { run } = polyfilled(file);

	it("are the polyfills, not the native methods", () => {
		expect(run("Array.prototype.at.toString()")).not.toContain("[native code]");
		expect(run("String.prototype.at.toString()")).not.toContain("[native code]");
	});

	it("index from either end", () => {
		expect(run("[1, 2, 3].at(-1)")).toBe(3);
		expect(run("[1, 2, 3].at(0)")).toBe(1);
		expect(run("[1, 2, 3].at(3)")).toBe(undefined);
		expect(run("'abc'.at(-1)")).toBe("c");
		expect(run("'abc'.at(0)")).toBe("a");
		expect(run("'abc'.at(3)")).toBe(undefined);
	});

	it("throw a TypeError on a null or undefined receiver", () => {
		for (const method of ["Array.prototype.at", "String.prototype.at"]) {
			for (const receiver of ["null", "undefined"]) {
				expect(
					run(`try { ${method}.call(${receiver}, 0); 'returned' } catch (e) { e instanceof TypeError }`),
					`${method}.call(${receiver}, 0)`,
				).toBe(true);
			}
		}
	});
});
