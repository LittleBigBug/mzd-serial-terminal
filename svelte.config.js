import preprocess from "svelte-preprocess";
import adapter from "@sveltejs/adapter-static";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			fallback: "index.html",
			pages: "./dist/html",
			assets: "./dist/html"
		}),
	},
	preprocess: preprocess(),
};

export default config;
