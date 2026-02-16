import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import path from 'path';

export default defineConfig({
	plugins: [svelte({ hot: !process.env.VITEST })],
	resolve: {
		alias: {
			'$lib': path.resolve('./src/lib'),
			'$lib/types': path.resolve('./src/lib/types'),
			'$lib/services': path.resolve('./src/lib/services'),
			'$lib/services/api': path.resolve('./src/lib/services/api'),
			'$lib/services/stores': path.resolve('./src/lib/services/stores'),
			'$lib/components': path.resolve('./src/lib/components'),
			'$lib/components/ui': path.resolve('./src/lib/components/ui'),
			'$lib/utils': path.resolve('./src/lib/utils'),
			'$lib/assets': path.resolve('./src/lib/assets'),
		}
	},
	test: {
		globals: true,
		environment: 'happy-dom',
		setupFiles: ['./tests/setup.ts'],
		include: ['src/**/*.{test,spec}.{js,ts}', 'tests/**/*.{test,spec}.{js,ts}'],
		coverage: {
			provider: 'v8',
			reporter: ['text', 'json', 'html'],
			exclude: [
				'node_modules/',
				'tests/',
				'*.config.*',
				'**/*.d.ts'
			]
		}
	}
});
