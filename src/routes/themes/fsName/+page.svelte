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
	import { goto } from "$app/navigation";
	import Chip from "../../../components/Chip.svelte";
	import FileIcon from "../../../components/svgs/FileIcon.svelte";
	import VersionIcon from "../../../components/svgs/VersionIcon.svelte";
	import TestIcon from "../../../components/svgs/TestIcon.svelte";
	import SensorIcon from "../../../components/svgs/SensorIcon.svelte";
	import FolderIcon from "../../../components/svgs/FolderIcon.svelte";
	import SliderIcon from "../../../components/svgs/SliderIcon.svelte";
	import { dragscroll } from "@svelte-put/dragscroll";

	function formatBytes(bytes: number | undefined, decimals = 2) {
		bytes = bytes ?? 0;
		if (!+bytes) return "0 Bytes";

		bytes = bytes * 1024;

		const k = 1024;
		const dm = decimals < 0 ? 0 : decimals;
		const sizes = ["Bytes", "KB", "MB", "GB"];

		const i = Math.floor(Math.log(bytes) / Math.log(k));

		return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
	}

	const getCoolerByShortName = (name: string): string => {
		switch (name) {
			case "capellix":
				return "the Corsair Capellix LCD";
			case "ttultra":
				return "the Thermaltake Toughliquid LCD";
			default:
				return "";
		}
	};

	let theme: Theme = {
		name: "undefined",
		fs_name: "",
		description: "",
		author: "",
		image_src: "",
		colour: "",
		customisable_parameters: [],
		tested_on: [],
		fileSizeKB: 0,
	};

	let networked = true;
	let loading = true;
	let fetching = false;
	let nowPlaying = false;

	let noDls = -1;

	let fsName = "";

	let installerListen: UnlistenFn;
	let uninstallerListen: UnlistenFn;

	onMount(async () => {
		fsName = $page.url.searchParams.get("fsName") ?? "";

		try {
			const { theme: themeMani, networked: net } = await returnLocalOrNetworked(fsName);
			theme = themeMani;
			console.log(theme);
			networked = net;
			if (!networked) {
				const documents = await documentDir();
				const appDir = await join(documents, "Zefir's Flashy Cooler");
				const preview_img = appDir + "\\" + theme.fs_name + "\\preview.jpg";
				theme.image_src = convertFileSrc(preview_img);
				nowPlaying = ((await invoke("now_serving")) as Theme).fs_name === fsName;
			}
			loading = false;
		} catch (e) {
			console.log(e);
			loading = false;
		}
		try {
			const manifest = await fetchAndParseTheme(fsName);
			console.log(manifest);
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
	<div class="chips-container" use:dragscroll>
		<div class="pad-elt" />
		{#if theme.fileSizeKB ?? 0 > 0}
			<Chip content={`${formatBytes(theme.fileSizeKB)}`}><FileIcon /></Chip>
		{/if}
		<Chip content={`v${theme.version}`}><VersionIcon /></Chip>
		{#each theme.tested_on as tested}
			<Chip content={`Tested on ${getCoolerByShortName(tested)}`}><TestIcon /></Chip>
		{/each}
		{#if theme.customisable_parameters.length > 0}
			<Chip content={`${theme.customisable_parameters.length} parameters`}><SliderIcon /></Chip>
		{/if}
		{#if theme.customisable_parameters.find((x) => x.type === "sensor")}
			<Chip content={`Uses sensors`}><SensorIcon /></Chip>
		{/if}
		{#if theme.customisable_parameters.find((x) => x.type === "sensor")}
			<Chip content={`Can use files`}><FolderIcon /></Chip>
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
					{#if nowPlaying}
						<button class="btn btn-outline-info" on:click={() => goto("/renderer")}>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								height="1em"
								viewBox="0 0 640 512"
								class="svg-info"
								><!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path
									d="M308.5 135.3c7.1-6.3 9.9-16.2 6.2-25c-2.3-5.3-4.8-10.5-7.6-15.5L304 89.4c-3-5-6.3-9.9-9.8-14.6c-5.7-7.6-15.7-10.1-24.7-7.1l-28.2 9.3c-10.7-8.8-23-16-36.2-20.9L199 27.1c-1.9-9.3-9.1-16.7-18.5-17.8C173.9 8.4 167.2 8 160.4 8h-.7c-6.8 0-13.5 .4-20.1 1.2c-9.4 1.1-16.6 8.6-18.5 17.8L115 56.1c-13.3 5-25.5 12.1-36.2 20.9L50.5 67.8c-9-3-19-.5-24.7 7.1c-3.5 4.7-6.8 9.6-9.9 14.6l-3 5.3c-2.8 5-5.3 10.2-7.6 15.6c-3.7 8.7-.9 18.6 6.2 25l22.2 19.8C32.6 161.9 32 168.9 32 176s.6 14.1 1.7 20.9L11.5 216.7c-7.1 6.3-9.9 16.2-6.2 25c2.3 5.3 4.8 10.5 7.6 15.6l3 5.2c3 5.1 6.3 9.9 9.9 14.6c5.7 7.6 15.7 10.1 24.7 7.1l28.2-9.3c10.7 8.8 23 16 36.2 20.9l6.1 29.1c1.9 9.3 9.1 16.7 18.5 17.8c6.7 .8 13.5 1.2 20.4 1.2s13.7-.4 20.4-1.2c9.4-1.1 16.6-8.6 18.5-17.8l6.1-29.1c13.3-5 25.5-12.1 36.2-20.9l28.2 9.3c9 3 19 .5 24.7-7.1c3.5-4.7 6.8-9.5 9.8-14.6l3.1-5.4c2.8-5 5.3-10.2 7.6-15.5c3.7-8.7 .9-18.6-6.2-25l-22.2-19.8c1.1-6.8 1.7-13.8 1.7-20.9s-.6-14.1-1.7-20.9l22.2-19.8zM112 176a48 48 0 1 1 96 0 48 48 0 1 1 -96 0zM504.7 500.5c6.3 7.1 16.2 9.9 25 6.2c5.3-2.3 10.5-4.8 15.5-7.6l5.4-3.1c5-3 9.9-6.3 14.6-9.8c7.6-5.7 10.1-15.7 7.1-24.7l-9.3-28.2c8.8-10.7 16-23 20.9-36.2l29.1-6.1c9.3-1.9 16.7-9.1 17.8-18.5c.8-6.7 1.2-13.5 1.2-20.4s-.4-13.7-1.2-20.4c-1.1-9.4-8.6-16.6-17.8-18.5L583.9 307c-5-13.3-12.1-25.5-20.9-36.2l9.3-28.2c3-9 .5-19-7.1-24.7c-4.7-3.5-9.6-6.8-14.6-9.9l-5.3-3c-5-2.8-10.2-5.3-15.6-7.6c-8.7-3.7-18.6-.9-25 6.2l-19.8 22.2c-6.8-1.1-13.8-1.7-20.9-1.7s-14.1 .6-20.9 1.7l-19.8-22.2c-6.3-7.1-16.2-9.9-25-6.2c-5.3 2.3-10.5 4.8-15.6 7.6l-5.2 3c-5.1 3-9.9 6.3-14.6 9.9c-7.6 5.7-10.1 15.7-7.1 24.7l9.3 28.2c-8.8 10.7-16 23-20.9 36.2L315.1 313c-9.3 1.9-16.7 9.1-17.8 18.5c-.8 6.7-1.2 13.5-1.2 20.4s.4 13.7 1.2 20.4c1.1 9.4 8.6 16.6 17.8 18.5l29.1 6.1c5 13.3 12.1 25.5 20.9 36.2l-9.3 28.2c-3 9-.5 19 7.1 24.7c4.7 3.5 9.5 6.8 14.6 9.8l5.4 3.1c5 2.8 10.2 5.3 15.5 7.6c8.7 3.7 18.6 .9 25-6.2l19.8-22.2c6.8 1.1 13.8 1.7 20.9 1.7s14.1-.6 20.9-1.7l19.8 22.2zM464 304a48 48 0 1 1 0 96 48 48 0 1 1 0-96z"
								/></svg
							>Show settings
						</button>
					{:else}
						<button
							class="btn btn-outline-success"
							on:click={() => {
								invoke("apply_theme", { fsName: theme.fs_name });
								nowPlaying = true;
							}}
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
					{/if}
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
