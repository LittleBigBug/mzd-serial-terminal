import { writable } from "svelte/store";

/**
 * @param {array} [initialState=[]]
 */
export const list = (initialState = []) => ({
	...writable(initialState ?? []),
	push(...values) {
		this.update((a) => a.concat(...values));
	},
});

/**
 * @param {string} initialState
 */
export const str = (initialState = "") => ({
	...writable(initialState ?? ""),
	add(...strings) {
		let addStr = "";
		for (const str of strings) addStr += str;
		this.update((v) => v + addStr);
	},
})