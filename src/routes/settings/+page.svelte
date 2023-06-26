<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	let themeFolder = "";

	let startLoginSwitch: HTMLInputElement;
	let startMinimisedSwitch: HTMLInputElement;

	let startAtLogin = false;
	let startMinimised = false;
	let pollRate = 3000;

	onMount(async () => {
		themeFolder = await invoke("get_theme_folder");

		startAtLogin = await invoke("get_start_login");
		startMinimised = await invoke("get_start_minimised");
		pollRate = await invoke("get_poll_rate");
	});

	const openThemeFolder = () => {
		invoke("open_theme_folder");
	};

	const handleLoginChange = async () => {
		await invoke("set_start_login", { setting: startLoginSwitch.checked });
	};

	const handleMinimisedChange = async () => {
		await invoke("set_start_minimised", { setting: startMinimisedSwitch.checked });
	};

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

<div class="actualSettings">
	<div id="about">
		<div class="card" style="width: 18rem;" id="zefirCard">
			<img src="../images/about_card.jpg" class="card-img-top" alt="Zefir" />
			<div class="card-body">
				<h5 class="card-title">Meet Zefir</h5>
				<p class="card-text">
					He's my cat. He supported me emotionally while making this app. I hope you enjoy this
					picture of him.
				</p>
			</div>
		</div>
		<h2>
			<img src="/images/android-chrome-192x192.png" id="logo" alt="Zefir's Flashy Cooler logo" /> Zefir's
			Flashy Cooler
		</h2>
		<p>Made with &#10084;&#65039; in Cork, Ireland.</p>
		<small
			>This software comes AS IS with no warranties or guaranties of ANY kind. If you feel it's
			shady, please check its source out on GitHub. Remember, this is third party software based on
			a reverse-engineering effort. If something breaks, sorry but I'm not responsible. I do my best
			so that this app works as intended and use it myself but I can't promise anything.</small
		>
		<small id="version" />
	</div>
	<hr />
	<div id="appSettings">
		<h3>App Settings</h3>
	</div>
	<div id="startAtLogin">
		<div class="form-check form-switch">
			<input
				class="form-check-input"
				type="checkbox"
				role="switch"
				id="loginSwitch"
				bind:this={startLoginSwitch}
				checked={startAtLogin}
				on:change={() => handleLoginChange()}
			/>
			<label class="form-check-label" for="loginSwitch">Start at login.</label>
			<br /><small
				>If checked, the app will start when you login to your user account on this system.</small
			>
		</div>
	</div>
	<div id="startMinimised">
		<div class="form-check form-switch">
			<input
				class="form-check-input"
				type="checkbox"
				role="switch"
				id="startMinimisedSwitch"
				bind:this={startMinimisedSwitch}
				checked={startMinimised}
				on:change={() => handleMinimisedChange()}
			/>
			<label class="form-check-label" for="startMinimisedSwitch">Start minimised.</label>
			<br /><small>If checked, the app will start minimised to system tray.</small>
		</div>
	</div>
	<div id="pollRate">
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
			The lower this value is, the more will sensor readings be up-to-date. This, however, comes
			with higher CPU usage.
		</small>
	</div>
	<hr />
	<div id="folders">
		<h3>Folders</h3>
		<button
			type="button"
			class="btn btn-outline-primary"
			id="themeFolderBtn"
			on:click={openThemeFolder}>Open theme folder</button
		>
		<br /><small class="theme-intro"
			>This is the folder in which themes are stored. To add one, just drag the downloaded theme
			into the theme folder. WARNING: Themes are full programs. Please make sure they're from a
			trusted source. They can do bad things to your computer. Trust but verify.</small
		>
		<br /><br /><small id="themeFolderText">Current theme folder:</small><br /><small
			>{themeFolder}</small
		>
	</div>
</div>

<style lang="scss">
	@import "./+page.scss";
</style>
