<script lang="ts">
	import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
	import { join, documentDir } from "@tauri-apps/api/path";
	import { onMount } from "svelte";

	interface Theme {
		name: string;
		fs_name: string;
		image_src: string;
	}

	interface PreTheme {
		name: string;
		fs_name: string;
	}

	let themes: Theme[] = [];
	let appDir: string;

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

		console.log(themes);
	});
</script>

<div id="everything">
	<div id="liveAlertPlaceholder" />

	<main id="card-container">
		{#each themes as theme}
			<article class="theme-card" id={theme.fs_name}>
				<img src={theme.image_src} alt={theme.name + " Card"} class="theme-img" />
				<h5 class="theme-title">
					{theme.name}
				</h5>
			</article>
		{/each}
	</main>
</div>

<style lang="scss">
	@import "./+page.scss";
	* {
		background-color: #00000000;
	}
</style>
