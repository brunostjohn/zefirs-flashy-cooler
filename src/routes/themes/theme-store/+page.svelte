<script lang="ts">
	import { DoubleBounce } from "svelte-loading-spinners";
	import NavigationCircle from "./../../../components/NavigationCircle.svelte";
	import SearchBar from "../../../components/SearchBar.svelte";
	import ThemeCard from "../../../components/ThemeCard.svelte";
	import { fade } from "svelte/transition";
	import type { Theme } from "../../../helpers/themeTools";
	import { onDestroy, onMount } from "svelte";
	import { getAllThemes } from "../../../helpers/ghApi";
	import { topBarMessage } from "../../../helpers/stores";

	let transitionLength = 200;

	let searchValue = "";

	let themes: Theme[] = [];
	let loading = true;

	onMount(async () => {
		try {
			themes = await getAllThemes();
		} catch (e) {
			console.log(e);
		}
		loading = false;
	});

	onDestroy(() => {
		topBarMessage.set("");
	});
</script>

<div id="everything">
	<div id="liveAlertPlaceholder" />
	<div class="title-header">
		<NavigationCircle href="../" />
		<h1>Theme Store</h1>
		{#if !loading && themes.length >= 0}
			<div class="search">
				<SearchBar placeholder="Search for themes" bind:input={searchValue} />
			</div>
		{/if}
	</div>
	{#if themes.length > 0 && searchValue.replace(" ", "").replace("	", "") === ""}
		<main id="card-container" in:fade={{ delay: transitionLength }}>
			{#each themes as theme}
				<ThemeCard src={theme.image_src} themeName={theme.name} fsName={theme.fs_name} />
			{/each}
			{#each [1, 2, 3, 4, 5, 6, 7, 8] as i}
				<article class="fillspace" />
			{/each}
		</main>
	{:else if themes.filter((theme) => theme.name.includes(searchValue)).length > 0}
		<main id="card-container" in:fade={{ delay: transitionLength }}>
			{#each themes.filter((theme) => theme.name.includes(searchValue)) as theme}
				<ThemeCard src={theme.image_src} themeName={theme.name} fsName={theme.fs_name} />
			{/each}
			{#each [1, 2, 3, 4, 5, 6, 7, 8] as _i}
				<article class="fillspace" />
			{/each}
		</main>
	{:else if loading}
		<div class="loading" transition:fade={{ duration: transitionLength - 30 }}>
			<h3>Loading</h3>
			<DoubleBounce color="#8bb9fe" />
		</div>
	{:else}
		<main id="no-themes" transition:fade={{ duration: transitionLength - 30 }}>
			{#if searchValue === ""}
				<h3>Failed to fetch themes.</h3>
				<p>Check your internet connection or try later.</p>
			{:else}
				<h3>Your search returned no results.</h3>
				<p>Try adjusting your search parameters or get more themes from the Theme Store.</p>
			{/if}
		</main>
	{/if}
</div>

<style lang="scss">
	@import "./+page.scss";
</style>
