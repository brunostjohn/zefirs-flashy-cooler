<script lang="ts">
	import { onMount } from "svelte";
	import type { Parameter } from "../../helpers/themeTools";
	import { invoke } from "@tauri-apps/api/tauri";
	import type ParameterValue from "./parameter";
	import { clickOutside } from "../../helpers/clickOutside";
	import { crossfade, fade } from "svelte/transition";

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

	let expand = false;
	const [send, recieve] = crossfade({ fallback: fade });
</script>

{#if expand}
	<div id="contents">
		<div
			id="container"
			use:clickOutside
			on:click_outside={() => (expand = false)}
			in:recieve={{ duration: 300 }}
			out:send={{ duration: 300 }}
		>
			<label for="inputText" class="form-label"><h5>{display_as}</h5></label>
			<input
				type="text"
				class="form-control"
				id="inputText"
				bind:this={input}
				on:change={handlerWrapper}
			/>
		</div>
	</div>
{:else}
	<div
		class="square"
		on:click={() => (expand = true)}
		in:recieve={{ duration: 300 }}
		out:send={{ duration: 300 }}
	>
		<h5>{display_as}</h5>
	</div>
{/if}

<style lang="scss">
	@import "../../styles/mixins.scss";

	#contents {
		position: fixed;
		top: 0;
		left: 0;

		width: 100%;
		height: 100%;
		backdrop-filter: blur(10px) brightness(0.8);

		z-index: 100;

		@include flex-center;
	}
	div {
		// margin-top: 1rem;

		input {
			background: none;
		}
	}

	.square {
		// margin-top: 1rem;

		width: 12rem;
		height: 12rem;
		border-radius: 15px;

		backdrop-filter: blur(10px);
		background-color: rgba(70, 117, 111, 0.4);

		@include flex-center;
	}
</style>
