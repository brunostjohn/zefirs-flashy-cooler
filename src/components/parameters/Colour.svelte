<script lang="ts">
	import { onMount } from "svelte";
	import type { Parameter } from "../../helpers/themeTools";
	import { invoke } from "@tauri-apps/api/tauri";
	import type ParameterValue from "./parameter";
	export let parameter: Parameter;
	import "@melloware/coloris/dist/coloris.css";
	import * as Coloris from "@melloware/coloris";

	let hex: string;

	const { name, display_as, default: def } = parameter;

	onMount(async () => {
		const parameterCurrent: ParameterValue = await invoke("get_current_theme_parameter", { name });
		hex = parameterCurrent.value.toLowerCase();
		Coloris.init();
		Coloris.coloris({
			el: "#coloris",
			theme: "pill",
			themeMode: "dark",
			alpha: false,
			onChange: handlerWrapper,
			defaultColor: hex,
		});

		hex = parameterCurrent.value.toLowerCase();
	});

	const updateConfig = async () => {
		await invoke("apply_theme_parameter", { name, value: hex });
	};

	// eslint-disable-next-line @typescript-eslint/no-empty-function
	let timeout = setTimeout(() => {}, 10);

	const handlerWrapper = () => {
		clearTimeout(timeout);
		timeout = setTimeout(updateConfig, 300);
	};
</script>

<div id="container">
	<h5>{display_as}</h5>
	<div id="pickerContainer">
		<input type="text" id="coloris" class="form-control" bind:value={hex} />
	</div>
</div>

<style lang="scss">
	div {
		margin-top: 1rem;

		#coloris {
			background-color: transparent;
			// width: 94.5vw;
		}
	}
</style>
