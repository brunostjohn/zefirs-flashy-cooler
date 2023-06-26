<script lang="ts">
	import { onMount } from "svelte";
	import type { Parameter } from "../../helpers/themeTools";
	import { invoke } from "@tauri-apps/api/tauri";
	import type ParameterValue from "./parameter";
	export let parameter: Parameter;

	let checked: boolean;

	const { name, display_as, default: def } = parameter;

	onMount(async () => {
		const parameterCurrent: ParameterValue = await invoke("get_current_theme_parameter", { name });

		checked = parameterCurrent.value === "true";
	});

	const updateConfig = async () => {
		await invoke("apply_theme_parameter", { name, value: `${checked}` });
	};
</script>

<h5>{display_as}</h5>
<div class="form-check form-switch">
	<input
		class="form-check-input"
		type="checkbox"
		role="switch"
		id="flexSwitchCheckDefault"
		bind:checked
		on:change={() => updateConfig()}
	/>
	<label class="form-check-label" for="flexSwitchCheckDefault">{checked ? "On" : "Off"}</label>
</div>

<style lang="scss">
	h5 {
		margin-top: 1rem;
	}
</style>
