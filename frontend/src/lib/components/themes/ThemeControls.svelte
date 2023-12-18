<script lang="ts">
	import type { Theme } from "$lib/stores/installedThemes";
	import { getModalStore, type ModalSettings } from "@skeletonlabs/skeleton";
	import IconButton from "../ui/IconButton.svelte";
	import ThemeCard from "./ThemeCard.svelte";
	import { Play, Trash, Cog6Tooth } from "@steeze-ui/heroicons";
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";

	export let theme: Theme;
	const modalStore = getModalStore();

	const handleUninstallTheme = async () => {
		await invoke("uninstall_theme_handler", { fsName: theme.fs_name });
	};

	const handleUninstallClick = () => {
		const modal: ModalSettings = {
			type: "confirm",
			title: "Are you sure?",
			body: "This will uninstall the theme and remove all its files. This cannot be reversed.",
			response: (r: boolean) => {
				if (r) {
					handleUninstallTheme().catch(console.error);
				}
			},
		};

		modalStore.trigger(modal);
	};

	let isThemePlaying = false;
	onMount(async () => {
		isThemePlaying = await invoke("is_theme_playing_handler", { fsName: theme.fs_name });
	});

	const handleThemePlay = async () => {
		try {
			await invoke("play_theme_handler", { fsName: theme.fs_name });
			isThemePlaying = true;
		} catch (e) {
			console.error("Failed to play theme", e);
		}
	};
</script>

<div class="w-fit">
	<ThemeCard {theme} isLink={false} dimensions="h-80 w-80" containerDimensions="w-80" />
	<div class="flex gap-4 mt-2">
		{#if isThemePlaying}
			<IconButton
				btnClass="btn variant-filled-tertiary w-1/2"
				src={Cog6Tooth}
				on:click={() => goto("/now-playing")}>Now Playing</IconButton
			>
		{:else}
			<IconButton btnClass="btn variant-filled-success w-1/2" src={Play} on:click={handleThemePlay}
				>Play</IconButton
			>
		{/if}
		<IconButton
			btnClass="btn variant-filled-warning w-1/2"
			src={Trash}
			on:click={handleUninstallClick}>Uninstall</IconButton
		>
	</div>
</div>
