<script lang="ts">
	import { onMount } from "svelte";
	import type { Parameter } from "../../helpers/themeTools";
	import { invoke } from "@tauri-apps/api/tauri";
	import type ParameterValue from "./parameter";
	export let parameter: Parameter;

	const { name, display_as, default: def, min, max, step } = parameter;

	let currentValue = 0;

	onMount(async () => {
		const parameterCurrent: ParameterValue = await invoke("get_current_theme_parameter", { name });
		currentValue = parseFloat(parameterCurrent.value);
	});

	const updateConfig = async () => {
		await invoke("apply_theme_parameter", { name, value: `${currentValue}` });
	};

	// eslint-disable-next-line @typescript-eslint/no-empty-function
	let timeout = setTimeout(() => {}, 10);

	const handlerWrapper = () => {
		if (currentValue > parseFloat(max)) {
			currentValue = parseFloat(max);
		} else if (currentValue < parseFloat(min)) {
			currentValue = parseFloat(min);
		}
		clearTimeout(timeout);
		timeout = setTimeout(updateConfig, 300);
	};
</script>

<label for="rangeInput" class="form-label"><h5>{display_as}</h5></label>
<div id="rangeContainer">
	<div class="input-group mb-3" id="direct">
		<span class="input-group-text" id="numberInput">Current value</span>
		<input
			type="number"
			{min}
			{max}
			{step}
			class="form-control"
			id="directInput"
			aria-describedby="numberInput"
			bind:value={currentValue}
			on:change={handlerWrapper}
		/>
	</div>
	<div id="rangeInput">
		<p>{min}</p>
		<input
			type="range"
			class="form-range"
			{min}
			{max}
			{step}
			id="pollRate"
			bind:value={currentValue}
			on:change={handlerWrapper}
		/>
		<p>{max}</p>
	</div>
</div>

<style lang="scss">
	label {
		margin-top: 1rem;
	}

	#rangeInput {
		margin-left: 1rem;
		display: flex;
		width: 100%;

		input {
			margin-left: 1rem;
			margin-right: 1rem;
		}
	}

	#numberInput {
		background: none;
	}

	#directInput {
		background: none;
	}

	#direct {
		max-width: 17rem;
	}

	#rangeContainer {
		display: flex;
		width: 100%;

		align-items: center;
	}
</style>
