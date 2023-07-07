<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	let pollRate = 3000;

	onMount(async () => {
		pollRate = await invoke("get_poll_rate");
	});

	// eslint-disable-next-line @typescript-eslint/no-empty-function
	let timeout = setTimeout(() => {}, 10);

	const handlerWrapper = () => {
		if (pollRate > 10000) {
			pollRate = 10000;
		} else if (pollRate < 1000) {
			pollRate = 1000;
		}
		clearTimeout(timeout);
		timeout = setTimeout(handlePollRateChange, 300);
	};

	const handlePollRateChange = async () => {
		await invoke("set_poll_rate", { pollRate });
	};
</script>

<h2>Sensors</h2>
<div id="pollRate" class="pollRateContainer">
	<label for="pollRate" class="form-label">Sensor poll rate.</label>
	<div id="pollRateFlex">
		<p>0s</p>
		<input
			type="range"
			class="form-range"
			min="1000"
			max="10000"
			step="1"
			id="pollRate"
			bind:value={pollRate}
			on:change={handlerWrapper}
		/>
		<p>10s</p>
	</div>
	<div class="input-group mb-3">
		<span class="input-group-text" id="pollrate">Current value in ms:</span>
		<input
			type="number"
			min="1000"
			max="10000"
			class="form-control"
			step="1"
			aria-describedby="pollrate"
			bind:value={pollRate}
			on:change={handlerWrapper}
		/>
	</div>
	<small>
		The lower this value is, the more will sensor readings be up-to-date. This, however, comes with
		higher CPU usage.
	</small>
</div>

<style lang="scss">
	@import "./+page.scss";
</style>
