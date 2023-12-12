<script lang="ts">
	import Shell from "$lib/components/shell/Shell.svelte";
	import "../app.postcss";

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from "@floating-ui/dom";
	import { storePopup } from "@skeletonlabs/skeleton";
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<Shell>
	<slot />
</Shell>
