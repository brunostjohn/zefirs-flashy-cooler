<script lang="ts">
	import { onDestroy, onMount } from "svelte";
	import type { Parameter } from "../../helpers/themeTools";
	import { invoke } from "@tauri-apps/api/tauri";
	import { listen } from "@tauri-apps/api/event";
	import type ParameterValue from "./parameter";
	import type { UnlistenFn } from "@tauri-apps/api/event";
	export let parameter: Parameter;

	let current: string;

	let doneUnlisten: UnlistenFn;

	const { name, display_as } = parameter;

	onMount(async () => {
		const parameterCurrent: ParameterValue = await invoke("get_current_theme_parameter", { name });

		current = parameterCurrent.value;

		doneUnlisten = await listen("changed-file", (event) => {
			const payload = event.payload as string;

			current = payload;
		});
	});

	onDestroy(async () => {
		await doneUnlisten();
	});

	const updateConfig = async () => {
		await invoke("select_file_and_save", { name, current });
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div id="container" on:click={updateConfig}>
	<h5>{display_as}</h5>
	<h6>Select file</h6>
</div>

<style lang="scss">
	@import "../../styles/mixins.scss";

	#container {
		// margin-top: 1rem;
		width: 12rem;
		height: 12rem;
		@include flex-center;
		flex-direction: column;
		border-radius: 15px;

		background-color: rgba(185, 121, 194, 0.205);
	}
</style>
