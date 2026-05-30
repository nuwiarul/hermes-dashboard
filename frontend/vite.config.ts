import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [tailwindcss(), sveltekit()],
    test: {
        environment: 'jsdom',
        include: ['src/**/*.{test,spec}.ts'],
        setupFiles: ['src/setupTests.ts'],
        globals: true,
    },
    server: {
        proxy: {
            '/api': {
                target: 'https://api-hermes.vinrul.my.id',
                changeOrigin: true,
                secure: true,
            },
            '/ws': {
                target: 'wss://api-hermes.vinrul.my.id',
                ws: true,
            },
        },
    },
});
