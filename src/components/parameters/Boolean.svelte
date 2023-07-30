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

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="elt"
	on:click={() => {
		checked = !checked;
		updateConfig();
	}}
	style={`background-color: ${checked ? "rgba(5, 109, 24, 0.5)" : "rgba(5, 109, 24, 0.2)"}`}
>
	<h5>{display_as}</h5>
	<h6>
		{checked ? "Yes" : "No"}
	</h6>
</div>

<style lang="scss">
	@import "../../styles/mixins.scss";

	.elt {
		width: 12rem;
		height: 12rem;

		@include flex-center;
		flex-direction: column;

		border-radius: 15px;

		transition: all 150ms ease-in-out;

		h5 {
			text-align: center;
			margin-bottom: 1rem;
		}
	}
</style>
