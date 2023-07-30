import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import GithubActionsReporter from "vitest-github-actions-reporter";

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		reporters: process.env.GITHUB_ACTIONS ? ["default", new GithubActionsReporter()] : "default",
	},
});
