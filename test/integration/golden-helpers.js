// The readable bundle lists every included polyfill in its explainer
// comment as " * - <Name>, License: ..." lines. That list is the resolved
// feature set for the requesting UA.
export function extractFeatures(rawBundle) {
	const comment = rawBundle.split("*/")[0];
	return [...comment.matchAll(/^ \* - ([^,]+), License/gm)].map(match => match[1]);
}
