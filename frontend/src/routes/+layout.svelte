<script lang="ts">
	import Shell from "$lib/components/shell/Shell.svelte";
	import "../app.postcss";
	import { attachConsole } from "tauri-plugin-log-api";

	onMount(() => {
		let detach = () => {};

		attachConsole()
			.then((dFn) => {
				console.log("Console attached");
				detach = dFn;
			})
			.catch((err) => {
				console.error("Failed to attach console", err);
			});

		return () => {
			console.log("Detaching console");
			detach();
			console.log("Console detached");
		};
	});

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from "@floating-ui/dom";
	import { storePopup } from "@skeletonlabs/skeleton";
	import { onMount } from "svelte";
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<Shell>
	<slot />
</Shell>
