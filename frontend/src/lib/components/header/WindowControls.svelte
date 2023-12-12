<script lang="ts">
	import Close from "./Controls/Close.svelte";
	import ExitApp from "./Controls/ExitApp.svelte";
	import Minimise from "./Controls/Minimise.svelte";
	import { appWindow } from "@tauri-apps/api/window";
	import { invoke } from "@tauri-apps/api/tauri";

	const handleWindowExit = async () => {
		await invoke("exit_handler");
	};
</script>

<div class="flex flex-row-reverse gap-2 p-[0.6rem] {$$props.class ?? ''}">
	<Close on:click={() => appWindow.close()} />
	<ExitApp on:click={() => handleWindowExit()} />
	<Minimise on:click={() => appWindow.minimize()} />
</div>
