import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [tailwindcss(), sveltekit()],
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
