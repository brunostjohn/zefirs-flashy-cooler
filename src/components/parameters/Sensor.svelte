<script lang="ts">
	import { onMount } from "svelte";
	import type { Parameter } from "../../helpers/themeTools";
	import { invoke } from "@tauri-apps/api/tauri";
	import type ParameterValue from "./parameter";
	import { each } from "svelte/internal";
	export let parameter: Parameter;

	interface Sensor {
		parent_hw_type: string;
		sensor: string;
		type: string;
		value: string;
	}

	interface FlattenedHardware {
		name: string;
		displayName: string;
		sensors: Sensor[];
	}

	interface Subhardware {
		name: string;
		sensors: Sensor[];
	}

	interface Hardware {
		name: string;
		subhardware: Subhardware[];
		sensors: Sensor[];
	}

	let input: HTMLInputElement;

	let allSensors: FlattenedHardware[] = [];

	let allSensorsPre: Hardware[] = [];

	let computedCategories: string[] = [];

	let parent: string;
	let category: string;

	const { name, display_as } = parameter;

	onMount(async () => {
		const parameterCurrent: ParameterValue = await invoke("get_current_theme_parameter", { name });

		allSensorsPre = (await invoke("get_all_sensors")) as Hardware[];

		processHardware();
	});

	const processHardware = () => {
		let temp = [];
		for (const hardware of allSensorsPre) {
			if (hardware.sensors.length > 0) {
				temp.push({
					name: hardware.name,
					sensors: hardware.sensors,
					displayName: hardware.name,
				});
			}

			if (hardware.subhardware !== null) {
				if (hardware.subhardware.length > 0) {
					for (const sub of hardware.subhardware) {
						temp.push({
							...sub,
							displayName: sub.name,
							name: `${hardware.name}/subhardware/${sub.name}`,
						});
					}
				}
			}
		}

		allSensors = temp;
	};

	const updateConfig = async () => {
		await invoke("apply_theme_parameter", { name, value: input.value });
	};

	const handleParentChange = () => {
		let branch = allSensors.filter((hardware) => {
			return hardware.name === parent;
		})[0];

		let temp: string[] = [];

		for (const sensor of branch.sensors) {
			if (!temp.includes(sensor.type)) {
				temp.push(sensor.type);
			}
		}

		computedCategories = temp;
	};
</script>

<div id="container">
	{#if allSensors.length > 0}
		<select bind:value={parent} on:change={handleParentChange} class="form-control">
			{#each allSensors as sensor}
				<option value={sensor.name}>{sensor.displayName}</option>
			{/each}
		</select>
		{#if computedCategories.length > 0}
			<select bind:value={category} class="form-control">
				{#each computedCategories as category}
					<option value={category}>{category}</option>
				{/each}
			</select>
		{/if}
	{/if}
</div>

<style lang="scss">
	div {
		margin-top: 1rem;
	}

	select {
		background: none;
	}
</style>
