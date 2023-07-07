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

<div id="container">
	<h5>{display_as}</h5>
	<div id="flex">
		<button class="btn btn-outline-primary" on:click={updateConfig}>Select file</button>
		<p>
			Currently selected: {current === "" || current === undefined ? "No file selected" : current}
		</p>
	</div>
</div>

<style lang="scss">
	#container {
		margin-top: 1rem;

		#flex {
			display: flex;
			align-items: center;

			p {
				margin-bottom: 0;
				margin-left: 1rem;
			}
		}
	}
</style>
