import { readFileSync } from "node:fs";
import { createContext, runInContext } from "node:vm";
import { describe, expect, it } from "vitest";

const LIBRARY = new URL("../../polyfill-libraries/5.3.1/", import.meta.url);

// String.prototype.replaceAll and the abstract operations it depends on, in
// dependency order. RegExp.prototype.flags and Symbol.replace are native in
// every browser that is served replaceAll, so they are not loaded.
const SOURCES = [
	"_ESAbstract.Call",
	"_ESAbstract.CreateMethodProperty",
	"_ESAbstract.Get",
	"_ESAbstract.ToObject",
	"_ESAbstract.GetV",
	"_ESAbstract.IsCallable",
	"_ESAbstract.GetMethod",
	"_ESAbstract.Type",
	"_ESAbstract.GetSubstitution",
	"_ESAbstract.ToBoolean",
	"_ESAbstract.IsRegExp",
	"_ESAbstract.RequireObjectCoercible",
	"_ESAbstract.StringIndexOf",
	"_ESAbstract.OrdinaryToPrimitive",
	"_ESAbstract.ToPrimitive",
	"_ESAbstract.ToString",
	"String.prototype.replaceAll",
];

/** A fresh global with the polyfill installed from `file`, like a browser without replaceAll. */
function polyfilled(file: "raw.js" | "min.js"): { run(code: string): unknown } {
	const context = createContext({});
	runInContext("var self = this; delete String.prototype.replaceAll;", context);
	for (const name of SOURCES) {
		runInContext(readFileSync(new URL(`${name}/${file}`, LIBRARY), "utf8"), context, {
			filename: `${name}/${file}`,
		});
	}
	return { run: (code) => runInContext(code, context) };
}

describe.each(["raw.js", "min.js"] as const)("String.prototype.replaceAll (%s)", (file) => {
	const { run } = polyfilled(file);

	it("is the polyfill, not the native method", () => {
		expect(run("String.prototype.replaceAll.toString()")).not.toContain("[native code]");
	});

	it("leaves a replacement without $ as it is", () => {
		expect(run("'abab'.replaceAll('b', '-')")).toBe("a-a-");
	});

	it("substitutes $$, $& and $'", () => {
		expect(run("'abab'.replaceAll('b', '$$')")).toBe("a$a$");
		expect(run("'abab'.replaceAll('b', '[$&]')")).toBe("a[b]a[b]");
		expect(run("'xyab'.replaceAll('a', \"[$']\")")).toBe("xy[b]b");
		expect(run("'abc'.replaceAll('b', '$')")).toBe("a$c");
	});

	it("substitutes $` with everything before the match", () => {
		expect(run("'xyab'.replaceAll('a', '[$`]')")).toBe("xy[xy]b");
		expect(run("'axa'.replaceAll('a', '[$`]')")).toBe("[]x[ax]");
	});

	it("keeps $n literal when a string search has no captures", () => {
		expect(run("'abc'.replaceAll('b', '$1')")).toBe("a$1c");
		expect(run("'abc'.replaceAll('b', '$9')")).toBe("a$9c");
		expect(run("'abc'.replaceAll('b', '$0')")).toBe("a$0c");
		expect(run("'abc'.replaceAll('b', '$01')")).toBe("a$01c");
		expect(run("'abc'.replaceAll('b', '$10')")).toBe("a$10c");
	});

	it("substitutes captures with a regex search", () => {
		expect(run("'aXbX'.replaceAll(/(X)/g, '<$1>')")).toBe("a<X>b<X>");
		expect(run("'aXbX'.replaceAll(/(X)/g, '$10')")).toBe("aX0bX0");
		expect(run("'abcdefghij'.replaceAll(/(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)/g, '$10')")).toBe("j");
	});

	describe("GetSubstitution with captures", () => {
		const substitute = (captures: string, replacement: string) =>
			run(`GetSubstitution('X', 'aXb', 1, ${captures}, undefined, ${JSON.stringify(replacement)})`);

		it("reads $n and $nn within the capture count", () => {
			expect(substitute("['c']", "<$1>")).toBe("<c>");
			expect(substitute("['c']", "<$01>")).toBe("<c>");
			expect(substitute("'abcdefghij'.split('')", "<$10>")).toBe("<j>");
		});

		it("reads $nn beyond the capture count as $n and a digit", () => {
			expect(substitute("['c']", "<$10>")).toBe("<c0>");
			expect(substitute("['c']", "<$11>")).toBe("<c1>");
		});

		it("keeps a reference beyond the capture count literal", () => {
			expect(substitute("['c']", "<$2>")).toBe("<$2>");
			expect(substitute("['c']", "<$00>")).toBe("<$00>");
			expect(substitute("['c']", "<$20>")).toBe("<$20>");
		});

		it("substitutes an undefined capture with the empty string", () => {
			expect(substitute("[undefined]", "<$1>")).toBe("<>");
		});
	});
});
