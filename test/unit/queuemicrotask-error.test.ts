import { readFileSync } from "node:fs";
import { createContext, runInContext } from "node:vm";
import { JSDOM, VirtualConsole } from "jsdom";
import { describe, expect, it } from "vitest";

const LIBRARY = new URL("../../polyfill-libraries/5.3.1/", import.meta.url);

type File = "raw.js" | "min.js";

const source = (file: File) => readFileSync(new URL(`queueMicrotask/${file}`, LIBRARY), "utf8");

/** Lets every queued microtask, and the reactions they queue, run. */
const settle = () => new Promise((resolve) => setTimeout(resolve, 20));

/** A jsdom window with the polyfill installed, like a browser without queueMicrotask. */
function polyfilledWindow(file: File) {
	const { window } = new JSDOM("", {
		runScripts: "outside-only",
		virtualConsole: new VirtualConsole(),
	});
	window.eval("delete self.queueMicrotask;");
	window.eval(source(file));
	return window;
}

describe.each(["raw.js", "min.js"] as const)("queueMicrotask (%s)", (file) => {
	it("is the polyfill, not the native function", () => {
		const window = polyfilledWindow(file);
		expect(window.eval("queueMicrotask.toString()")).not.toContain("[native code]");
	});

	it("runs callbacks in order, after the current task", async () => {
		const window = polyfilledWindow(file);
		window.eval(`
			var log = [];
			queueMicrotask(function () { log.push('A'); });
			Promise.resolve().then(function () { log.push('promise'); });
			queueMicrotask(function () { log.push('B'); });
			log.push('sync');
		`);
		await settle();
		expect(window.eval("log")).toEqual(["sync", "A", "promise", "B"]);
	});

	it("reports a throwing callback as an uncaught error before the next callback runs", async () => {
		const window = polyfilledWindow(file);
		window.eval(`
			var log = [];
			var errorB = new Error('B');
			var reported;
			self.addEventListener('error', function (event) {
				reported = event.error;
				log.push('error ' + event.error.message);
			});
			queueMicrotask(function () { log.push('A'); });
			queueMicrotask(function () { log.push('B'); throw errorB; });
			queueMicrotask(function () { log.push('C'); });
		`);
		await settle();
		expect(window.eval("log")).toEqual(["A", "B", "error B", "C"]);
		expect(window.eval("reported === errorB")).toBe(true);
	});

	it("rethrows a throwing callback from a timer without a document", async () => {
		const timers: (() => void)[] = [];
		const context = createContext({
			setTimeout: (callback: () => void) => timers.push(callback),
			// A worker has Event and dispatchEvent, but no document.
			Event: function Event() {},
			dispatchEvent: () => true,
		});
		runInContext("var self = this; delete self.queueMicrotask;", context);
		runInContext(source(file), context, { filename: `queueMicrotask/${file}` });
		runInContext(
			`
			var log = [];
			var errorB = new Error('B');
			queueMicrotask(function () { log.push('A'); });
			queueMicrotask(function () { log.push('B'); throw errorB; });
			queueMicrotask(function () { log.push('C'); });
		`,
			context,
		);
		await settle();
		expect(runInContext("log", context)).toEqual(["A", "B", "C"]);
		expect(timers).toHaveLength(1);
		expect(timers[0]).toThrow(runInContext("errorB", context) as Error);
	});
});
