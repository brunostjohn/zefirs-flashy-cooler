<script lang="ts">
	import { DoubleBounce } from "svelte-loading-spinners";
	import { onDestroy, onMount } from "svelte";
	import SvelteMarkdown from "svelte-markdown";
	import HeadingRendererMd from "../../components/HeadingRendererMd.svelte";
	import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
	import { documentDir, join } from "@tauri-apps/api/path";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import Boolean from "../../components/parameters/Boolean.svelte";
	import type { Theme } from "../../helpers/themeTools";
	import Sensor from "../../components/parameters/Sensor.svelte";
	import Range from "../../components/parameters/Range.svelte";
	import Colour from "../../components/parameters/Colour.svelte";
	import Text from "../../components/parameters/Text.svelte";
	import File from "../../components/parameters/File.svelte";

	let theme: Theme = {
		name: "undefined",
		fs_name: "",
		description: "",
		author: "",
		image_src: "",
		colour: "",
		customisable_parameters: [],
		tested_on: [],
	};

	let loading = true;
	let defaultTheme = false;

	let uninstallerListen: UnlistenFn;

	onMount(async () => {
		try {
			theme = await invoke("now_serving");
			loading = false;
			const documents = await documentDir();
			const appDir = await join(documents, "Zefir's Flashy Cooler");
			const preview_img = appDir + "\\" + theme.fs_name + "\\preview.jpg";
			theme.image_src = convertFileSrc(preview_img);
			if (theme.fs_name === "__DEFAULT__") {
				defaultTheme = true;
				theme.image_src = "/images/default.png";
			}
		} catch (e) {
			console.log(e);
		}

		uninstallerListen = await listen("delete-successful", (_) => {
			window.location.reload();
		});
	});

	onDestroy(async () => {
		await uninstallerListen();
	});
</script>

{#if loading}
	<div class="title-header">
		<div class="theme-collect">
			<h1>Loading...</h1>
		</div>
	</div>
	<div class="loading-container">
		<DoubleBounce color="#8bb9fe" />
	</div>
{:else if theme.name === "undefined"}
	<div class="title-header">
		<div class="theme-collect">
			<h1>Failed to load.</h1>
		</div>
	</div>
	<div class="loading-container" />
{:else}
	<div class="info-content">
		<div class="theme-collect">
			<h1>{theme.name}</h1>
			<h3>by {theme.author}</h3>
			<SvelteMarkdown source={theme.description} renderers={{ heading: HeadingRendererMd }} />
		</div>
		<div class="preview-card">
			<img src={theme.image_src} alt="theme preview" class="preview-image" />
			{#if !defaultTheme}
				<div class="btn-group">
					<button
						class="btn btn-outline-danger"
						on:click={() => invoke("uninstall_theme", { fsName: theme.fs_name })}
					>
						<svg
							class="svg-danger"
							xmlns="http://www.w3.org/2000/svg"
							height="1em"
							viewBox="0 0 448 512"
							><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
								d="M135.2 17.7L128 32H32C14.3 32 0 46.3 0 64S14.3 96 32 96H416c17.7 0 32-14.3 32-32s-14.3-32-32-32H320l-7.2-14.3C307.4 6.8 296.3 0 284.2 0H163.8c-12.1 0-23.2 6.8-28.6 17.7zM416 128H32L53.2 467c1.6 25.3 22.6 45 47.9 45H346.9c25.3 0 46.3-19.7 47.9-45L416 128z"
							/></svg
						>Delete from drive
					</button>
				</div>
			{/if}
		</div>
	</div>
	{#if theme.customisable_parameters.length > 0}
		<div class="controllable">
			<hr />
			<h2>Customisable parameters</h2>
			{#each theme.customisable_parameters as parameter}
				{#if parameter.type === "boolean"}
					<Boolean {parameter} />
				{:else if parameter.type === "sensor"}
					<Sensor {parameter} />
				{:else if parameter.type === "range"}
					<Range {parameter} />
				{:else if parameter.type === "colour"}
					<Colour {parameter} />
				{:else if parameter.type === "text"}
					<Text {parameter} />
				{:else if parameter.type === "file"}
					<File {parameter} />
				{/if}
			{/each}
		</div>
	{/if}
{/if}

<style lang="scss">
	@import "./+page.scss";
</style>
