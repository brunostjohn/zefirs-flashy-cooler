<script lang="ts">
	import { themeStore, type Theme } from "$lib/stores/installedThemes";
	import ThemeControls from "$lib/components/themes/ThemeControls.svelte";
	import SvelteMarkdown from "svelte-markdown";

	export let data;
	$: ({ fsName } = data);

	$: theme =
		$themeStore.find((theme) => theme.fs_name === fsName) ??
		({
			name: "",
			author: "",
			description: "",
			tested_on: [],
			version: "",
		} as unknown as Theme);
	$: ({ name, author, description, tested_on, version } = theme);
</script>

<div class="flex">
	<div class="mr-auto w-full">
		<div class="flex align-center items-center">
			<div>
				<h1 class="h1">{name}</h1>
				<h2 class="h5 text-surface-400">by {author}</h2>
			</div>
			<div class="ml-auto mr-5 flex flex-col gap-1">
				<small class="inline-flex align-center items-center text-surface-300"
					>Version: <code class="code ml-1">{version}</code></small
				>
				<small class="inline-flex align-center items-center text-surface-300"
					>Tested on: <code class="code ml-1">{tested_on.join(" ,")}</code></small
				>
			</div>
		</div>
		<div class="mt-4 mr-5">
			<SvelteMarkdown source={description} />
		</div>
	</div>
	<ThemeControls {theme} />
</div>
