/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: 'class',
	content: ['./src/**/*.{html,rs}', 'index.html', 'index.css'],

	theme: {
		extend: {},
	},
	plugins: [
		require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
	],
};
