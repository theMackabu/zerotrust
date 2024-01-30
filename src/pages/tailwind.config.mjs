/** @type {import('tailwindcss').Config} */
export default {
	content: ['./safelist.tmp', './src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
		extend: { fontFamily: { sans: ['Inter var', ...require('tailwindcss/defaultTheme').fontFamily.sans] } }
	},
	plugins: [
		require('@tailwindcss/forms'),
		require('tailwind-safelist-generator')({
			path: 'safelist.tmp',
			patterns: [
				'text-{colors}',
				'border-{colors}',
				'bg-{colors}',
				'hover:text-{colors}',
				'hover:border-{colors}',
				'hover:bg-{colors}',
				'focus:ring-{colors}',
				'focus-visible:outline-{colors}'
			]
		})
	]
};
