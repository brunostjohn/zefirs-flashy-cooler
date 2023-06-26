<script lang="ts">
	import { DoubleBounce } from "svelte-loading-spinners";
	import { page } from "$app/stores";
	import { onDestroy, onMount } from "svelte";
	import NavigationCircle from "../../../components/NavigationCircle.svelte";
	import { returnLocalOrNetworked, type Theme } from "../../../helpers/themeTools";
	import SvelteMarkdown from "svelte-markdown";
	import HeadingRendererMd from "../../../components/HeadingRendererMd.svelte";
	import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
	import { documentDir, join } from "@tauri-apps/api/path";
	import ThemeDownload from "../../../components/ThemeDownload.svelte";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { fetchAndParseTheme } from "../../../helpers/ghApi";

	let theme: Theme = {
		name: "undefined",
		fs_name: "",
		description: "",
		author: "",
		image_src: "",
		colour: "",
		customisable_parameters: [],
	};

	let networked = true;
	let loading = true;
	let fetching = false;

	let noDls = -1;

	let fsName = "";

	let installerListen: UnlistenFn;
	let uninstallerListen: UnlistenFn;

	onMount(async () => {
		fsName = $page.url.searchParams.get("fsName") ?? "";

		try {
			const themeData = await returnLocalOrNetworked(fsName);
			theme = themeData.theme;
			networked = themeData.networked;
			if (!networked) {
				const documents = await documentDir();
				const appDir = await join(documents, "Zefir's Flashy Cooler");
				const preview_img = appDir + "\\" + theme.fs_name + "\\preview.jpg";
				theme.image_src = convertFileSrc(preview_img);
			}
			loading = false;
		} catch (e) {
			console.log(e);
			loading = false;
		}
		try {
			const manifest = await fetchAndParseTheme(fsName);
			noDls = manifest.dls !== undefined ? manifest.dls : -1;
		} catch (e) {
			console.log(e);
		}

		installerListen = await listen("theme-installed", async (_) => {
			fetching = false;
			networked = false;

			try {
				await new Promise<void>((resolve) => setTimeout(() => resolve(), 300));
				const manifest = await fetchAndParseTheme(fsName);
				noDls = manifest.dls !== undefined ? manifest.dls : -1;
			} catch (e) {
				console.log(e);
			}
		});

		uninstallerListen = await listen("delete-successful", (_) => {
			networked = true;
		});
	});

	onDestroy(async () => {
		await installerListen();
		await uninstallerListen();
	});

	const installTheme = () => {
		fetching = true;
		invoke("install_theme", { fsName: theme.fs_name });
	};
</script>

{#if fetching}
	<ThemeDownload />
{/if}
{#if loading}
	<div class="title-header">
		<NavigationCircle href="../" />
		<div class="theme-collect">
			<h1>Loading...</h1>
		</div>
	</div>
	<div class="loading-container">
		<DoubleBounce color="#8bb9fe" />
	</div>
{:else if theme.name === "undefined"}
	<div class="title-header">
		<NavigationCircle href="../" />
		<div class="theme-collect">
			<h1>Failed to load.</h1>
		</div>
	</div>
	<div class="loading-container" />
{:else}
	<div class="title-header">
		<NavigationCircle href="../" />
		<div class="theme-collect">
			<h1>{theme.name}</h1>
			<h3>by {theme.author}</h3>
		</div>
		{#if noDls > -1}
			<h4 class="dls">{noDls} downloads</h4>
		{/if}
	</div>

	<div class="info-content">
		<div class="preview-card">
			<img src={theme.image_src} alt="theme preview" class="preview-image" />
			<div class="btn-group">
				{#if networked}
					<button class="btn btn-outline-primary" on:click={() => installTheme()}>
						<svg
							class="svg-primary"
							xmlns="http://www.w3.org/2000/svg"
							height="1em"
							viewBox="0 0 640 512"
							><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
								d="M144 480C64.5 480 0 415.5 0 336c0-62.8 40.2-116.2 96.2-135.9c-.1-2.7-.2-5.4-.2-8.1c0-88.4 71.6-160 160-160c59.3 0 111 32.2 138.7 80.2C409.9 102 428.3 96 448 96c53 0 96 43 96 96c0 12.2-2.3 23.8-6.4 34.6C596 238.4 640 290.1 640 352c0 70.7-57.3 128-128 128H144zm79-167l80 80c9.4 9.4 24.6 9.4 33.9 0l80-80c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-39 39V184c0-13.3-10.7-24-24-24s-24 10.7-24 24V318.1l-39-39c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9z"
							/></svg
						> Download Theme
					</button>
				{:else}
					<button
						class="btn btn-outline-success"
						on:click={() => invoke("apply_theme", { fsName: theme.fs_name })}
					>
						<svg
							class="svg-success"
							xmlns="http://www.w3.org/2000/svg"
							height="1em"
							viewBox="0 0 384 512"
							><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
								d="M73 39c-14.8-9.1-33.4-9.4-48.5-.9S0 62.6 0 80V432c0 17.4 9.4 33.4 24.5 41.9s33.7 8.1 48.5-.9L361 297c14.3-8.7 23-24.2 23-41s-8.7-32.2-23-41L73 39z"
							/></svg
						>Play on cooler
					</button>
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
				{/if}
			</div>
		</div>
		<SvelteMarkdown source={theme.description} renderers={{ heading: HeadingRendererMd }} />
	</div>
{/if}

<style lang="scss">
	@import "./+page.scss";
</style>
