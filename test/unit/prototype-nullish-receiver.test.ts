import { readdirSync, readFileSync } from "node:fs";
import { createContext, runInContext, type Context } from "node:vm";
import { describe, expect, it } from "vitest";

const LIBRARY = new URL("../../polyfill-libraries/5.3.1/", import.meta.url);

const FILES = ["raw.js", "min.js"] as const;
type File = (typeof FILES)[number];

const FIXED = [
	"Array.prototype.findLast",
	"Array.prototype.findLastIndex",
	"Array.prototype.toReversed",
	"Array.prototype.toSorted",
	"Array.prototype.toSpliced",
	"Array.prototype.with",
	"String.prototype.isWellFormed",
	"String.prototype.toWellFormed",
];

// Still return a value on a null or undefined receiver: https://github.com/puckey/polyfill-service/issues/8
const KNOWN_FAILURES = [
	"Array.prototype.entries",
	"Array.prototype.fill",
	"Array.prototype.find",
	"Array.prototype.findIndex",
	"Array.prototype.keys",
	"Iterator.prototype.drop",
	"Iterator.prototype.filter",
	"Iterator.prototype.flatMap",
	"Iterator.prototype.map",
	"Iterator.prototype.take",
	"RegExp.prototype.flags",
	"String.prototype.codePointAt",
	"TypedArray.prototype.at",
	"TypedArray.prototype.entries",
	"TypedArray.prototype.findLast",
	"TypedArray.prototype.findLastIndex",
	"TypedArray.prototype.keys",
	"TypedArray.prototype.sort",
	"TypedArray.prototype.toLocaleString",
];

// As above, in min.js only: its raw.js has a 'use strict' that min.js lacks.
const KNOWN_FAILURES_MIN_ONLY = [
	"Array.prototype.copyWithin",
	"Array.prototype.flat",
	"Array.prototype.includes",
	"Object.prototype.toString",
	"String.prototype.endsWith",
	"String.prototype.includes",
	"String.prototype.matchAll",
	"String.prototype.normalize",
	"String.prototype.padEnd",
	"String.prototype.padStart",
	"String.prototype.repeat",
	"String.prototype.startsWith",
	"String.prototype.trimEnd",
	"String.prototype.trimStart",
];

const NOT_PROBED: Record<string, string> = {
	"Function.prototype.name": "each function's own data property in the spec, not a method with a receiver",
};

/** Arguments that get each method to its receiver step; the default is a callback and 0. */
const ARGS: Record<string, string> = {
	"Iterator.prototype.drop": "1",
	"Iterator.prototype.take": "1",
	"Set.prototype.difference": "new Set()",
	"Set.prototype.intersection": "new Set()",
	"Set.prototype.isDisjointFrom": "new Set()",
	"Set.prototype.isSubsetOf": "new Set()",
	"Set.prototype.isSupersetOf": "new Set()",
	"Set.prototype.symmetricDifference": "new Set()",
	"Set.prototype.union": "new Set()",
	"String.prototype.normalize": "",
};

/** What the spec returns on a null and an undefined receiver, where it does not throw a TypeError. */
const SPEC_RESULTS: Record<string, [string, string]> = {
	"Object.prototype.toString": ["[object Null]", "[object Undefined]"],
};

// These polyfills call the native they replace, so it stays in place.
const KEEPS_NATIVE = ["Array.prototype.sort", "Object.prototype.toString"];

type Meta = { dependencies?: string[]; detectSource?: string };

function meta(name: string): Meta {
	return JSON.parse(readFileSync(new URL(`${name}/meta.json`, LIBRARY), "utf8"));
}

function prototypeOf(owner: string): string {
	return owner === "TypedArray" ? "Object.getPrototypeOf(Int8Array.prototype)" : `${owner}.prototype`;
}

function freshContext(): Context {
	const context = createContext({});
	runInContext("var self = this;", context);
	return context;
}

/** Every `<Owner>.prototype.<method>` feature whose owner a node vm global has. */
function scan(): { name: string; owner: string; method: string }[] {
	const context = freshContext();
	return readdirSync(LIBRARY)
		.sort()
		.flatMap((name) => {
			const match = /^(\w+)\.prototype\.(\w+)$/.exec(name);
			if (!match) {
				return [];
			}
			const [, owner, method] = match;
			const constructor = owner === "TypedArray" ? "Int8Array" : owner;
			return runInContext(`typeof ${constructor} === 'function'`, context) ? [{ name, owner, method }] : [];
		});
}

/** `name` and its dependencies, each after what it depends on. Throws on a cycle. */
function dependencyOrder(name: string): string[] {
	const order: string[] = [];
	const visiting = new Set<string>();
	const visit = (feature: string) => {
		if (order.includes(feature)) {
			return;
		}
		if (visiting.has(feature)) {
			throw new Error(`dependency cycle through ${feature}`);
		}
		visiting.add(feature);
		for (const dependency of meta(feature).dependencies ?? []) {
			visit(dependency);
		}
		visiting.delete(feature);
		order.push(feature);
	};
	visit(name);
	return order;
}

/**
 * A fresh global without the native `name`, and the bundle the service would
 * send for it from `file`: its dependencies whose detect fails, then `name`,
 * in the service's closure.
 */
function polyfilled(name: string, file: File): { run(code: string): unknown } {
	const [owner, , method] = name.split(".");
	const context = freshContext();
	if (!KEEPS_NATIVE.includes(name)) {
		runInContext(`delete ${prototypeOf(owner)}.${method};`, context);
	}
	const sources = dependencyOrder(name)
		.filter((feature) => {
			const { detectSource } = meta(feature);
			return feature === name || !detectSource || !runInContext(detectSource, context);
		})
		.map((feature) => readFileSync(new URL(`${feature}/${file}`, LIBRARY), "utf8"));
	runInContext(`(function(self, undefined) {\n${sources.join("\n")}\n})(this);`, context, {
		filename: `${name}/${file}`,
	});
	return { run: (code) => runInContext(code, context) };
}

/** The method (or getter) `name` called on null and on undefined: "TypeError" or what it returned or threw. */
function nullishOutcomes(name: string, file: File): string[] {
	const [owner, , method] = name.split(".");
	const { run } = polyfilled(name, file);
	return run(`(function () {
		var descriptor = Object.getOwnPropertyDescriptor(${prototypeOf(owner)}, ${JSON.stringify(method)});
		var fn = descriptor.get || descriptor.value;
		return [null, undefined].map(function (receiver) {
			try {
				return 'returned ' + String(fn.call(receiver, ${ARGS[name] ?? "function () { return true; }, 0"}));
			} catch (error) {
				return error instanceof TypeError ? 'TypeError' : 'threw ' + error;
			}
		});
	})()`) as string[];
}

function expectedOutcomes(name: string): string[] {
	const results = SPEC_RESULTS[name];
	return results ? results.map((result) => `returned ${result}`) : ["TypeError", "TypeError"];
}

const FEATURES = scan().filter(({ name }) => !(name in NOT_PROBED));

it("names only features the scan probes", () => {
	const probed = FEATURES.map(({ name }) => name);
	expect(probed).toEqual(expect.arrayContaining([...FIXED, ...KNOWN_FAILURES, ...KNOWN_FAILURES_MIN_ONLY]));
	expect(scan().map(({ name }) => name)).toEqual(expect.arrayContaining(Object.keys(NOT_PROBED)));
});

describe.each(FILES)("prototype polyfills on a null or undefined receiver (%s)", (file) => {
	const knownFailures = file === "min.js" ? [...KNOWN_FAILURES, ...KNOWN_FAILURES_MIN_ONLY] : KNOWN_FAILURES;

	for (const { name } of FEATURES) {
		if (knownFailures.includes(name)) {
			it(`${name} still does not throw (issue #8)`, () => {
				expect(nullishOutcomes(name, file)).not.toEqual(expectedOutcomes(name));
			});
		} else {
			it(`${name} ${name in SPEC_RESULTS ? "returns what the spec says" : "throws a TypeError"}`, () => {
				expect(nullishOutcomes(name, file)).toEqual(expectedOutcomes(name));
			});
		}
	}
});

describe.each(FILES)("the fixed prototype polyfills (%s)", (file) => {
	const install = (name: string) => polyfilled(name, file).run;

	it.each(FIXED)("%s is the polyfill, not the native method", (name) => {
		expect(install(name)(`${name}.toString()`)).not.toContain("[native code]");
	});

	it("work on an ordinary receiver", () => {
		expect(install("Array.prototype.findLast")("[1, 2, 3].findLast(function (x) { return x < 3; })")).toBe(2);
		expect(install("Array.prototype.findLastIndex")("[1, 2, 3].findLastIndex(function (x) { return x < 3; })")).toBe(1);
		expect(install("Array.prototype.toReversed")("[1, 2, 3].toReversed().join()")).toBe("3,2,1");
		expect(install("Array.prototype.toSorted")("[3, 1, 2].toSorted().join()")).toBe("1,2,3");
		expect(install("Array.prototype.toSpliced")("[1, 2, 3].toSpliced(1, 1).join()")).toBe("1,3");
		expect(install("Array.prototype.with")("[1, 2, 3].with(1, 5).join()")).toBe("1,5,3");
		expect(install("String.prototype.isWellFormed")("'ab'.isWellFormed()")).toBe(true);
		expect(install("String.prototype.toWellFormed")("'a\\uD800'.toWellFormed()")).toBe("a�");
	});

	it("work on a primitive receiver", () => {
		expect(install("Array.prototype.findLast")("Array.prototype.findLast.call('abc', function (c) { return c < 'c'; })")).toBe("b");
		expect(
			install("Array.prototype.findLastIndex")("Array.prototype.findLastIndex.call('abc', function (c) { return c < 'c'; })"),
		).toBe(1);
		expect(install("Array.prototype.toReversed")("Array.prototype.toReversed.call('abc').join('')")).toBe("cba");
		expect(install("Array.prototype.toSorted")("Array.prototype.toSorted.call('cab').join('')")).toBe("abc");
		expect(install("Array.prototype.toSpliced")("Array.prototype.toSpliced.call('abc', 1, 1).join('')")).toBe("ac");
		expect(install("Array.prototype.with")("Array.prototype.with.call('abc', 0, 'x').join('')")).toBe("xbc");
		expect(install("String.prototype.isWellFormed")("String.prototype.isWellFormed.call(42)")).toBe(true);
		expect(install("String.prototype.toWellFormed")("String.prototype.toWellFormed.call(42)")).toBe("42");
	});
});
