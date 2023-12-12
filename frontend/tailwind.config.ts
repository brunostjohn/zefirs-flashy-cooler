import { join } from "path";
import type { Config } from "tailwindcss";
import forms from "@tailwindcss/forms";
import typography from "@tailwindcss/typography";
import { skeleton } from "@skeletonlabs/tw-plugin";
import { zfcTheme } from "./theme.js";

export default {
	jit: true,
	darkMode: "class",
	content: [
		"./src/**/*.{html,js,svelte,ts}",
		join(require.resolve("@skeletonlabs/skeleton"), "../**/*.{html,js,svelte,ts}"),
	],
	theme: {
		extend: {},
	},
	plugins: [
		forms,
		typography,
		skeleton({
			themes: {
				custom: [zfcTheme],
			},
		}),
	],
} satisfies Config;
