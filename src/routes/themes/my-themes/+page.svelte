<script lang="ts">
	import NavigationCircle from "./../../../components/NavigationCircle.svelte";
	import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
	import { join, documentDir } from "@tauri-apps/api/path";
	import { onMount } from "svelte";
	import SearchBar from "../../../components/SearchBar.svelte";
	import ThemeCard from "../../../components/ThemeCard.svelte";
	import { fade } from "svelte/transition";

	let transitionLength = 200;

	interface Theme {
		name: string;
		fs_name: string;
		image_src: string;
		colour: string;
	}

	interface PreTheme {
		name: string;
		fs_name: string;
		colour: string;
	}

	let themes: Theme[] = [];
	let appDir: string;

	let searchValue = "";

	onMount(async () => {
		const documents = await documentDir();
		appDir = await join(documents, "Zefir's Flashy Cooler");
		const pre_themes: PreTheme[] = await invoke("get_all_themes");

		console.log(pre_themes);

		themes = pre_themes.map((theme) => {
			const preview_img = appDir + "\\" + theme.fs_name + "\\preview.jpg";
			(theme as Theme)["image_src"] = convertFileSrc(preview_img);
			return theme;
		}) as Theme[];

		themes = themes.sort((a, b) => {
			if (a.name < b.name) {
				return -1;
			}
			if (a.name > b.name) {
				return 1;
			}
			return 0;
		});
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
	{#if themes.length > 0 && searchValue.replace(" ", "").replace("	", "") === ""}
		<main id="card-container" in:fade={{ delay: transitionLength }}>
			{#each themes as theme}
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
	{:else if themes.filter((theme) => theme.name.includes(searchValue)).length > 0}
		<main id="card-container" in:fade={{ delay: transitionLength }}>
			{#each themes.filter((theme) => theme.name.includes(searchValue)) as theme}
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
