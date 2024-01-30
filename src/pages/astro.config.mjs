import replace from './replace';
import tailwind from '@astrojs/tailwind';
import { defineConfig } from 'astro/config';

export default defineConfig({
	build: { format: 'file', assets: 'assets_provider' },
	integrations: [
		tailwind(),
		replace({
			__login_status: '{{button_status}}',
			__is_checked: '{{remember_checked}}',
			'/assets_provider/': '/{{prefix}}/assets/'
		})
	]
});
