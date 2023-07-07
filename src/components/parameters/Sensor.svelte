<script lang="ts">
	import { onMount } from "svelte";
	import type { Parameter } from "../../helpers/themeTools";
	import { invoke } from "@tauri-apps/api/tauri";
	import type ParameterValue from "./parameter";
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

	let allSensors: FlattenedHardware[] = [];

	let allSensorsPre: Hardware[] = [];

	let computedCategories: string[] = [];
	let computedSensors: { name: string; displayName: string }[] = [];

	let parent: string;
	let category: string;
	let selectedPath: string;

	const { name, display_as } = parameter;

	const initial = (parameterCurrent: ParameterValue) => {
		if (parameterCurrent.value) {
			if (parameterCurrent.value.length > 0) {
				processHardware();

				const path = parameterCurrent.value.split("/");

				parent = path[0];
				handleParentChange();

				category = path[1] === "subhardware" ? path[2] : path[1];
				handleCategoryChange();

				selectedPath = parameterCurrent.value;
			}
		}
	};

	onMount(async () => {
		const parameterCurrent: ParameterValue = await invoke("get_current_theme_parameter", { name });

		allSensorsPre = (await invoke("get_all_sensors")) as Hardware[];

		processHardware();

		initial(parameterCurrent);
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
		console.log(selectedPath);
		await invoke("apply_theme_parameter", { name, value: selectedPath });
	};

	const handleParentChange = () => {
		computedSensors = [];

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
		category = computedCategories[0];
		handleCategoryChange();
	};

	const handleCategoryChange = () => {
		let branch = allSensors.filter((hardware) => {
			return hardware.name === parent;
		})[0];

		let temp = branch.sensors.filter((x) => x.type === category);

		let temp_built = [];

		let path = `${parent}/${category}/`;

		for (const sensor of temp) {
			const obj = {
				displayName: sensor.sensor,
				name: path + sensor.sensor,
			};

			temp_built.push(obj);
		}

		computedSensors = temp_built;
		selectedPath = computedSensors[0].name;
	};
</script>

<div id="container">
	<h5>{display_as}</h5>
	{#if allSensors.length > 0}
		<label for="parent" id="parentLbl">Hardware</label>
		<select bind:value={parent} on:change={handleParentChange} class="form-control" id="parent">
			{#each allSensors as sensor}
				<option value={sensor.name}>{sensor.displayName}</option>
			{/each}
		</select>
		{#if computedCategories.length > 0}
			<label for="cat">Category</label>
			<select bind:value={category} on:change={handleCategoryChange} class="form-control" id="cat">
				{#each computedCategories as category}
					<option value={category}>{category}</option>
				{/each}
			</select>
		{/if}
		{#if computedSensors.length > 0}
			<label for="sensor">Sensor</label>
			<select bind:value={selectedPath} on:change={updateConfig} class="form-control" id="sensor">
				{#each computedSensors as sensor}
					<option value={sensor.name}>{sensor.displayName}</option>
				{/each}
			</select>
		{/if}
	{/if}
</div>

<style lang="scss">
	#parentLbl {
		margin-top: 0;
	}

	div {
		margin-top: 1rem;

		label {
			margin-top: 1rem;
			margin-bottom: 0.5rem;
		}

		select {
			background-color: transparent;

			option {
				background-color: var(--bs-body-bg);
			}
		}
	}
</style>
