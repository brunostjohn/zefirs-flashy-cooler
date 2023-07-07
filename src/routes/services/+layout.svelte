<script lang="ts">
	import ServiceOption from "../../components/ServiceOption.svelte";
	import SensorIcon from "../../components/svgs/SensorIcon.svelte";
</script>

<main>
	<div class="col-opts">
		<ServiceOption href="" name="Sensors" desc="See your CPU's temperature on your cooler."
			><SensorIcon /></ServiceOption
		>
	</div>
	<div class="content">
		<slot />
	</div>
</main>

<style lang="scss">
	@import "./+layout.scss";
</style>
