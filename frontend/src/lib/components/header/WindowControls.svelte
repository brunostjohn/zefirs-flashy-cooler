<script lang="ts">
	import Close from "./Controls/Close.svelte";
	import ExitApp from "./Controls/ExitApp.svelte";
	import Minimise from "./Controls/Minimise.svelte";
	import { appWindow } from "@tauri-apps/api/window";
	import { invoke } from "@tauri-apps/api/tauri";
	import { saveWindowState, StateFlags } from "tauri-plugin-window-state-api";
	import { trace } from "tauri-plugin-log-api";

	const handleWindowExit = async () => {
		trace("Exiting app");
		await invoke("activity_handler", { setTo: "clear" });
		await invoke("exit_handler");
	};

	const handleWindowClose = async () => {
		await invoke("activity_handler", { setTo: "clear" });
		trace("Closing window");
		trace("Saving window state");
		await saveWindowState(StateFlags.ALL);
		trace("Window state saved");
		trace("Closing window");
		await appWindow.close();
	};

	const handleWindowMinimise = async () => {
		trace("Minimising window");
		await appWindow.minimize();
	};
</script>

<div class="flex flex-row-reverse gap-2 p-[0.6rem] {$$props.class ?? ''}">
	<Close on:click={() => handleWindowClose()} />
	<ExitApp on:click={() => handleWindowExit()} />
	<Minimise on:click={() => handleWindowMinimise()} />
</div>
