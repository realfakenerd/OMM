/// <reference types="vitest" />
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import tailwindcss from '@tailwindcss/vite';
import vitePluginCssMediaSplitter from "css-media-splitter/vite-plugin"
import deadFile from 'vite-plugin-deadfile';
import { qrcode } from 'vite-plugin-qrcode';

export default defineConfig({
    plugins: [qrcode(), tailwindcss(), deadFile({ root: 'src' }), sveltekit(), vitePluginCssMediaSplitter()],
    css: {
        transformer: 'lightningcss',
    },
    json: {
        stringify: true
    },
    build: {
        reportCompressedSize: false,
        cssMinify: 'lightningcss',
        minify: 'terser',
        terserOptions: {
            compress: {
                drop_console: true,
                drop_debugger: true,
                ecma: 2020,
                module: true,
                toplevel: true,
                passes: 2,
                pure_getters: true,
                sequences: true,
                dead_code: true,
                unused: true
            },
            mangle: {
                toplevel: true,
                module: true
            },
            format: {
                comments: false,
                ecma: 2020
            }
        }
    },
    resolve: {
        conditions: process.env.VITEST ? ['browser'] : []
    },
    test: {
        environment: 'jsdom',
        setupFiles: ['./tests/setup.ts'],
        include: ['src/**/*.{test,spec}.{js,ts}', 'tests/**/*.{test,spec}.{js,ts}']
    }
});
