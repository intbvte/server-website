import typography from "@tailwindcss/typography"

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			fontFamily: {
				minecraft: 'Minecraft'
			},
			textShadow: {
				// sm: '0 1px var(--tw-shadow-color)',
				DEFAULT: '2px 2px 0 var(--tw-shadow-color)'
				// lg: '0 8px 16px var(--tw-shadow-color)'
			},
			colors: {
				sky: '#add8e6',
				gold: '#e4b763',
				lightgray: '#c6c6c6',
				darkred: '#582424',
				goldhighlight: '#ffeb8c',
				gray: '#8b8b8b'
			}
		}
	},
	plugins: [
		function ({ matchUtilities, theme }) {
			matchUtilities(
				{
					'text-shadow': (value) => ({
						textShadow: value
					})
				},
				{ values: theme('textShadow') }
			);
		},
		typography
	]
};
