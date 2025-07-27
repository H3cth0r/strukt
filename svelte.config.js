import adapter from '@sveltejs/adapter-static';

import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

// This will be '/strukt' during the GitHub Actions build, and '/' otherwise.
const dev = process.argv.includes('dev');
const base = dev ? '' : process.env.BASE_PATH || '';

/** @type {import('@sveltejs/kit').Config} */
const config = {
preprocess: vitePreprocess(),

kit: {
	adapter: adapter({
		// default options are fine
		pages: 'build',
		assets: 'build',
		fallback: undefined,
		precompress: false,
		strict: true
	}),
	// ✅ ADD THIS PATHS CONFIGURATION
	paths: {
		base: base,
	}
}
};

export default config;
