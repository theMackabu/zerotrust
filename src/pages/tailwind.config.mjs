/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
		extend: { fontFamily: { sans: ['Inter var', ...require('tailwindcss/defaultTheme').fontFamily.sans] } }
	},
	safelist: [{ pattern: /(text|bg|border)-(\w+)-(\w+)/ }],
	plugins: [require('@tailwindcss/forms')]
};
