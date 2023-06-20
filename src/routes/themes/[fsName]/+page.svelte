<script lang="ts">
	import { page } from "$app/stores";
	import { onMount } from "svelte";
	import NavigationCircle from "../../../components/NavigationCircle.svelte";
	import { returnLocalOrNetworked, type Theme } from "../../../helpers/themeTools";
	import SvelteMarkdown from "svelte-markdown";
	import HeadingRendererMd from "../../../components/HeadingRendererMd.svelte";
	import { invoke } from "@tauri-apps/api/tauri";

	let theme: Theme = {
		name: "",
		fs_name: "",
		description: "",
		author: "",
		image_src: "",
		colour: "",
	};

	let networked = true;
	let loading = true;

	const fsName = $page.params.fsName;

	onMount(async () => {
		try {
			const themeData = await returnLocalOrNetworked(fsName);
			theme = themeData.theme;
			// networked = themeData.networked;
			loading = false;
		} catch (e) {
			console.log(e);
			loading = false;
		}
	});
</script>

<div class="title-header">
	<NavigationCircle href="../" />
	<div class="theme-collect">
		<h1>{theme.name}</h1>
		<h3>by {theme.author}</h3>
	</div>
	<div class="btn-group">
		{#if networked}
			<button
				class="btn btn-outline-primary"
				on:click={() => invoke("install_theme", { fsName: theme.fs_name })}
			>
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

<div class="info-content">
	<div class="preview-card">
		<img src={theme.image_src} alt="theme preview" class="preview-image" />
	</div>
	<SvelteMarkdown source={theme.description} renderers={{ heading: HeadingRendererMd }} />
</div>

<style lang="scss">
	@import "./+page.scss";
</style>
