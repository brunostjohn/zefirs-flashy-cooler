<script lang="ts">
	import NavigationCircle from "./../../../components/NavigationCircle.svelte";
	import SearchBar from "../../../components/SearchBar.svelte";
	import ThemeCard from "../../../components/ThemeCard.svelte";
	import { fade } from "svelte/transition";
	import { onMount } from "svelte";
	import { getThemes, type Theme } from "../../../helpers/themeTools";

	let transitionLength = 200;

	let searchValue = "";

	let data = { themes: [] as Theme[] };

	onMount(async () => {
		data.themes = await getThemes();
	});
</script>

<div id="everything">
	<div id="liveAlertPlaceholder" />
	<div class="title-header">
		<NavigationCircle href="../" />
		<h1>My Themes</h1>
		<div class="search">
			<SearchBar placeholder="Search for themes" bind:input={searchValue} />
		</div>
	</div>
	{#if data.themes.length > 0 && searchValue.replace(" ", "").replace("	", "") === ""}
		<main id="card-container" in:fade={{ delay: transitionLength }}>
			{#each data.themes as theme}
				<ThemeCard
					src={theme.image_src}
					themeName={theme.name}
					textColour={theme.colour}
					fsName={theme.fs_name}
				/>
			{/each}
			{#each [1, 2, 3, 4, 5, 6, 7, 8] as i}
				<article class="fillspace" />
			{/each}
		</main>
	{:else if data.themes.filter((theme) => theme.name.includes(searchValue)).length > 0}
		<main id="card-container" in:fade={{ delay: transitionLength }}>
			{#each data.themes.filter((theme) => theme.name.includes(searchValue)) as theme}
				<ThemeCard
					src={theme.image_src}
					themeName={theme.name}
					textColour={theme.colour}
					fsName={theme.fs_name}
				/>
			{/each}
			{#each [1, 2, 3, 4, 5, 6, 7, 8] as _i}
				<article class="fillspace" />
			{/each}
		</main>
	{:else}
		<main id="no-themes" transition:fade={{ duration: transitionLength - 30 }}>
			{#if searchValue === ""}
				<h3>No themes found.</h3>
				<p>Try downloading some from the Theme Store.</p>
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
