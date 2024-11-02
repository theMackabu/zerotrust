import replace from './replace';
import react from '@astrojs/react';
import tailwind from '@astrojs/tailwind';
import { defineConfig } from 'astro/config';

export default defineConfig({
	build: { format: 'file', assets: 'assets_provider' },
	integrations: [tailwind(), react(), replace({ '/assets_provider/': '/{{prefix}}/assets/' })],
	vite: { build: { rollupOptions: { output: { assetFileNames: 'assets_provider/[hash].[ext]' } } } }
});
