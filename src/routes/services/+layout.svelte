<script lang="ts">
	import ServiceOption from "../../components/ServiceOption.svelte";
	import SrgbIcon from "../../components/svgs/SRGBIcon.svelte";
	import SensorIcon from "../../components/svgs/SensorIcon.svelte";
	import { fly } from "svelte/transition";
	import { cubicIn, cubicOut } from "svelte/easing";

	const duration = 200;
	const delay = duration + 50;
	const y = 10;

	export let data;

	const transitionIn = { easing: cubicOut, y, duration, delay };
	const transitionOut = { easing: cubicIn, y: -y, duration };
</script>

<main>
	<div class="col-opts">
		<ServiceOption href="" name="Sensors" desc="See your CPU's temperature on your cooler."
			><SensorIcon /></ServiceOption
		>
		<!-- <ServiceOption
			href="/srgb"
			name="SignalRGB"
			desc="Sync your cooler with your PC's lighting effects."
		>
			<SrgbIcon />
		</ServiceOption> -->
	</div>
	{#key data.pathname}
		<div class="content" in:fly={transitionIn} out:fly={transitionOut}>
			<slot />
		</div>
	{/key}
</main>

<style lang="scss">
	@import "./+layout.scss";
</style>
