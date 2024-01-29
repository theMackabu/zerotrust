import tailwind from '@astrojs/tailwind';
import { defineConfig } from 'astro/config';

export default defineConfig({
	integrations: [tailwind()],
	build: { format: 'file', assets: '_sp/assets' }
});
