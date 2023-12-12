/* eslint-env node */
const autoprefixer = require("autoprefixer");
const tailwindcss = require("tailwindcss");

/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: "class",
	content: [
		"./src/**/*.{html,js,svelte,ts}",
		// 3. Append the path to the Skeleton package
		require("path").join(require.resolve("@skeletonlabs/skeleton"), "../**/*.{html,js,svelte,ts}"),
	],
	theme: {
		extend: {},
	},
	plugins: [tailwindcss, autoprefixer],
};
