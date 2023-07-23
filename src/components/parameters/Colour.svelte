<script lang="ts">
	import { onMount } from "svelte";
	import type { Parameter } from "../../helpers/themeTools";
	import { invoke } from "@tauri-apps/api/tauri";
	import type ParameterValue from "./parameter";
	export let parameter: Parameter;
	import "@melloware/coloris/dist/coloris.css";
	import * as Coloris from "@melloware/coloris";
	import { crossfade, fade } from "svelte/transition";
	import { clickOutside } from "../../helpers/clickOutside";

	let hex: string;
	let rgb = { r: 255, g: 255, b: 255 };

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
		rgb = hexToRgb(hex);
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

	function hexToRgb(hex: string): number {
		// Expand shorthand form (e.g. "03F") to full form (e.g. "0033FF")
		var shorthandRegex = /^#?([a-f\d])([a-f\d])([a-f\d])$/i;
		hex = hex.replace(shorthandRegex, function (m, r, g, b) {
			return r + r + g + g + b + b;
		});

		var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
		return result
			? {
					r: parseInt(result[1], 16),
					g: parseInt(result[2], 16),
					b: parseInt(result[3], 16),
			  }
			: null;
	}

	let expand = false;
	const [send, recieve] = crossfade({ fallback: fade });
</script>

{#if expand}
	<div id="container" transition:fade={{ duration: 150 }}>
		<div
			class="items"
			use:clickOutside
			on:click_outside={() => (expand = false)}
			in:recieve={{ duration: 300 }}
			out:send={{ duration: 300 }}
		>
			<h5>{display_as}</h5>
			<div id="pickerContainer">
				<input type="text" id="coloris" class="form-control" bind:value={hex} />
			</div>
		</div>
	</div>
{:else}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="small-icon"
		style={`background-color: rgba(${rgb.r}, ${rgb.g}, ${rgb.b}, 0.2)`}
		on:click={() => (expand = true)}
		in:recieve={{ duration: 300 }}
		out:send={{ duration: 300 }}
	>
		<h5>{display_as}</h5>
	</div>
{/if}

<style lang="scss">
	@import "../../styles/mixins.scss";

	#container {
		position: fixed;
		top: 0;
		left: 0;

		width: 100%;
		height: 100%;

		z-index: 100;

		backdrop-filter: blur(10px) brightness(0.8);

		@include flex-center;
	}

	.small-icon {
		// margin-top: 1rem;

		width: 12rem;
		height: 12rem;
		border-radius: 15px;

		backdrop-filter: blur(10px);

		@include flex-center;

		text-align: center;

		// #coloris {
		// 	background-color: transparent;
		// 	// width: 94.5vw;
		// }
	}
</style>
