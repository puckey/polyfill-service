import process from "node:process";

export const baseURL = process.env.HOST || "http://127.0.0.1:7676";

export interface TestResponse {
	status: number;
	headers: Headers;
	body: string;
}

export interface RequestOptions {
	method?: "GET" | "HEAD" | "OPTIONS" | "POST";
	/** Sent as the User-Agent header when defined — including the empty string. */
	userAgent?: string;
}

// Thin wrapper over node's built-in fetch. Like the axios client it
// replaced: no redirect following, no throwing on HTTP error statuses
// (fetch only throws on network errors), automatic decompression.
export async function request(path: string, options: RequestOptions = {}): Promise<TestResponse> {
	const headers = new Headers();
	if (options.userAgent !== undefined) {
		headers.set("user-agent", options.userAgent);
	}
	const response = await fetch(baseURL + path, {
		method: options.method ?? "GET",
		headers,
		redirect: "manual",
	});
	return {
		status: response.status,
		headers: response.headers,
		body: await response.text(),
	};
}
