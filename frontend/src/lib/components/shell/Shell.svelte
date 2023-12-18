<script lang="ts">
	import AppHeader from "../header/AppHeader.svelte";
	import { fly } from "svelte/transition";
	import { cubicIn, cubicOut } from "svelte/easing";

	export let pathname: URL["pathname"] = "";

	const duration = 300;
	const delay = duration + 100;
	const y = 10;

	const transitionIn = { easing: cubicOut, y, duration, delay };
	const transitionOut = { easing: cubicIn, y: -y, duration };
</script>

<AppHeader />
{#key pathname}
	<main class="mt-14 p-4" in:fly={transitionIn} out:fly={transitionOut}>
		<slot />
	</main>
{/key}
