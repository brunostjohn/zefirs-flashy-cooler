<script lang="ts">
	import ThemeCard from "$lib/components/themes/ThemeCard.svelte";
	import { themeStore } from "$lib/stores/installedThemes";
</script>

<div class="grid grid-cols-4 w-full">
	{#each $themeStore as theme}
		<ThemeCard {theme} />
	{/each}
</div>
