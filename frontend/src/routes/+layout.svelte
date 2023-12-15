<script lang="ts">
	import { documentDir, join } from "@tauri-apps/api/path";
	import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
	import Shell from "$lib/components/shell/Shell.svelte";
	import "../app.postcss";
	import { attachConsole } from "tauri-plugin-log-api";
	import { initializeStores } from "@skeletonlabs/skeleton";
	import { listen } from "@tauri-apps/api/event";
	import type { ToastEvent } from "$lib/types/toastEvent";

	initializeStores();
	const toastStore = getToastStore();

	const handleMount = async () => {
		const detach = await attachConsole();
		const fetchedThemes = await invoke<Theme[]>("get_all_themes_handler");
		const documents = await documentDir();
		const appDir = await join(documents, "Zefir's Flashy Cooler", "Themes");
		const updatedThemes = fetchedThemes.map((theme) => {
			const preview_img = appDir + "\\" + theme.fs_name + "\\preview.jpg";
			theme.image_src = convertFileSrc(preview_img);
			return theme;
		});
		const unlisten = await listen<ToastEvent>("toast_event", (event) => {
			const {
				payload: { message, background, timeout, autohide },
			} = event;
			const newEvent: ToastSettings = {
				message,
				background: background ?? "bg-surface-500",
				timeout: timeout ?? 3000,
				autohide: autohide ?? true,
			};

			toastStore.trigger(newEvent);
		});

		themeStore.set(updatedThemes);

		return () => {
			detach();
			unlisten();
		};
	};

	onMount(() => {
		let detach = () => {};

		handleMount()
			.then((detachFn) => {
				detach = detachFn;
			})
			.catch(console.error);

		return () => {
			console.log("Detaching console");
			detach();
			console.log("Console detached");
		};
	});

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from "@floating-ui/dom";
	import {
		Modal,
		Toast,
		getToastStore,
		storePopup,
		type ToastSettings,
	} from "@skeletonlabs/skeleton";
	import { onMount } from "svelte";
	import { themeStore, type Theme } from "$lib/stores/installedThemes";
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<Shell>
	<Modal />
	<Toast />
	<slot />
</Shell>
