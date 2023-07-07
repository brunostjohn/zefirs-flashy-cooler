<script lang="ts">
	import { onMount } from "svelte";
	import type { Parameter } from "../../helpers/themeTools";
	import { invoke } from "@tauri-apps/api/tauri";
	import type ParameterValue from "./parameter";
	export let parameter: Parameter;

	let input: HTMLInputElement;

	const { name, display_as, default: def } = parameter;

	onMount(async () => {
		const parameterCurrent: ParameterValue = await invoke("get_current_theme_parameter", { name });

		input.value = parameterCurrent.value;
	});

	const updateConfig = async () => {
		await invoke("apply_theme_parameter", { name, value: input.value });
	};

	// eslint-disable-next-line @typescript-eslint/no-empty-function
	let timeout = setTimeout(() => {}, 10);

	const handlerWrapper = () => {
		clearTimeout(timeout);
		timeout = setTimeout(updateConfig, 300);
	};
</script>

<div id="container">
	<label for="inputText" class="form-label"><h5>{display_as}</h5></label>
	<input
		type="text"
		class="form-control"
		id="inputText"
		bind:this={input}
		on:change={handlerWrapper}
	/>
</div>

<style lang="scss">
	div {
		margin-top: 1rem;

		input {
			background: none;
		}
	}
</style>
