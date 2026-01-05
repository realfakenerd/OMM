/// <reference types="vitest" />
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	resolve: process.env.VITEST
		? {
				conditions: ['browser']
			}
		: undefined,
	test: {
		environment: 'jsdom',
		setupFiles: ['./tests/setup.ts'],
		include: ['src/**/*.{test,spec}.{js,ts}', 'tests/**/*.{test,spec}.{js,ts}']
	}
});
