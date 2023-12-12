import type { CustomThemeConfig } from "@skeletonlabs/tw-plugin";

export const zfcTheme: CustomThemeConfig = {
	name: "zfcTheme",
	properties: {
		// =~= Theme Properties =~=
		"--theme-font-family-base": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		"--theme-font-family-heading": `ui-serif, Georgia, Cambria, 'Times New Roman', Times, serif`,
		"--theme-font-color-base": "0 0 0",
		"--theme-font-color-dark": "255 255 255",
		"--theme-rounded-base": "9999px",
		"--theme-rounded-container": "12px",
		"--theme-border-base": "1px",
		// =~= Theme On-X Colors =~=
		"--on-primary": "0 0 0",
		"--on-secondary": "255 255 255",
		"--on-tertiary": "0 0 0",
		"--on-success": "0 0 0",
		"--on-warning": "0 0 0",
		"--on-error": "255 255 255",
		"--on-surface": "255 255 255",
		// =~= Theme Colors  =~=
		// primary | #fb7ec1
		"--color-primary-50": "254 236 246", // #feecf6
		"--color-primary-100": "254 229 243", // #fee5f3
		"--color-primary-200": "254 223 240", // #fedff0
		"--color-primary-300": "253 203 230", // #fdcbe6
		"--color-primary-400": "252 165 212", // #fca5d4
		"--color-primary-500": "251 126 193", // #fb7ec1
		"--color-primary-600": "226 113 174", // #e271ae
		"--color-primary-700": "188 95 145", // #bc5f91
		"--color-primary-800": "151 76 116", // #974c74
		"--color-primary-900": "123 62 95", // #7b3e5f
		// secondary | #942afe
		"--color-secondary-50": "239 223 255", // #efdfff
		"--color-secondary-100": "234 212 255", // #ead4ff
		"--color-secondary-200": "228 202 255", // #e4caff
		"--color-secondary-300": "212 170 255", // #d4aaff
		"--color-secondary-400": "180 106 254", // #b46afe
		"--color-secondary-500": "148 42 254", // #942afe
		"--color-secondary-600": "133 38 229", // #8526e5
		"--color-secondary-700": "111 32 191", // #6f20bf
		"--color-secondary-800": "89 25 152", // #591998
		"--color-secondary-900": "73 21 124", // #49157c
		// tertiary | #0EA5E9
		"--color-tertiary-50": "219 242 252", // #dbf2fc
		"--color-tertiary-100": "207 237 251", // #cfedfb
		"--color-tertiary-200": "195 233 250", // #c3e9fa
		"--color-tertiary-300": "159 219 246", // #9fdbf6
		"--color-tertiary-400": "86 192 240", // #56c0f0
		"--color-tertiary-500": "14 165 233", // #0EA5E9
		"--color-tertiary-600": "13 149 210", // #0d95d2
		"--color-tertiary-700": "11 124 175", // #0b7caf
		"--color-tertiary-800": "8 99 140", // #08638c
		"--color-tertiary-900": "7 81 114", // #075172
		// success | #84cc16
		"--color-success-50": "237 247 220", // #edf7dc
		"--color-success-100": "230 245 208", // #e6f5d0
		"--color-success-200": "224 242 197", // #e0f2c5
		"--color-success-300": "206 235 162", // #ceeba2
		"--color-success-400": "169 219 92", // #a9db5c
		"--color-success-500": "132 204 22", // #84cc16
		"--color-success-600": "119 184 20", // #77b814
		"--color-success-700": "99 153 17", // #639911
		"--color-success-800": "79 122 13", // #4f7a0d
		"--color-success-900": "65 100 11", // #41640b
		// warning | #EAB308
		"--color-warning-50": "252 244 218", // #fcf4da
		"--color-warning-100": "251 240 206", // #fbf0ce
		"--color-warning-200": "250 236 193", // #faecc1
		"--color-warning-300": "247 225 156", // #f7e19c
		"--color-warning-400": "240 202 82", // #f0ca52
		"--color-warning-500": "234 179 8", // #EAB308
		"--color-warning-600": "211 161 7", // #d3a107
		"--color-warning-700": "176 134 6", // #b08606
		"--color-warning-800": "140 107 5", // #8c6b05
		"--color-warning-900": "115 88 4", // #735804
		// error | #D41976
		"--color-error-50": "249 221 234", // #f9ddea
		"--color-error-100": "246 209 228", // #f6d1e4
		"--color-error-200": "244 198 221", // #f4c6dd
		"--color-error-300": "238 163 200", // #eea3c8
		"--color-error-400": "225 94 159", // #e15e9f
		"--color-error-500": "212 25 118", // #D41976
		"--color-error-600": "191 23 106", // #bf176a
		"--color-error-700": "159 19 89", // #9f1359
		"--color-error-800": "127 15 71", // #7f0f47
		"--color-error-900": "104 12 58", // #680c3a
		// surface | #2b2b2b
		"--color-surface-50": "223 223 223", // #dfdfdf
		"--color-surface-100": "213 213 213", // #d5d5d5
		"--color-surface-200": "202 202 202", // #cacaca
		"--color-surface-300": "170 170 170", // #aaaaaa
		"--color-surface-400": "107 107 107", // #6b6b6b
		"--color-surface-500": "43 43 43", // #2b2b2b
		"--color-surface-600": "39 39 39", // #272727
		"--color-surface-700": "32 32 32", // #202020
		"--color-surface-800": "26 26 26", // #1a1a1a
		"--color-surface-900": "21 21 21", // #151515
	},
};
