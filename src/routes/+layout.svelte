<script lang="ts">
	import "bootstrap/dist/css/bootstrap.min.css";
	import "./background.css";
	import "bootstrap/dist/js/bootstrap.bundle";
	import { appWindow } from "@tauri-apps/api/window";
	import { invoke } from "@tauri-apps/api/tauri";
	import { page } from "$app/stores";

	const murder = async () => {
		await invoke("remote_exit");
	};

	import { fly } from "svelte/transition";
	import { cubicIn, cubicOut } from "svelte/easing";

	export let data;

	const duration = 200;
	const delay = duration + 50;
	const y = 10;

	const transitionIn = { easing: cubicOut, y, duration, delay };
	const transitionOut = { easing: cubicIn, y: -y, duration };
</script>

<div id="topbar" data-tauri-drag-region>
	<img
		src="/images/android-chrome-192x192.png"
		id="titlebarlogo"
		alt="Zefir's Flashy Cooler Logo"
	/>
	<nav class="nav nav-pills nav-fill" id="controlbar">
		<a class="nav-link" href="/" class:active={$page.url.pathname === "/"}>Themes</a>
		<a class="nav-link" href="/device" class:active={$page.url.pathname === "/device"}>Device</a>
		<a class="nav-link" href="/renderer" class:active={$page.url.pathname === "/renderer"}
			>Renderer</a
		>
		<a class="nav-link" href="/settings" class:active={$page.url.pathname === "/settings"}
			>Settings</a
		>
	</nav>
	<div id="titlebarcontrols">
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<a id="minimise" on:click={() => appWindow.minimize()}>
			<img
				src="/images/window-minimize-solid.svg"
				id="minimisewindowimage"
				alt="minimise window button"
			/>
		</a>
		<div id="spacerTitle" />
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<a id="closeapp" on:click={murder}>
			<img
				src="/images/arrow-right-from-bracket-solid.svg"
				id="killAppImage"
				alt="exit app button"
			/>
		</a>
		<div id="spacerTitle" />
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<a id="closewindow" on:click={() => appWindow.close()}>
			<img src="/images/closewindow.svg" id="closewindowimage" alt="close window button" />
		</a>
	</div>
</div>
<div id="everything">
	<div id="liveAlertPlaceholder" />

	<main id="card-container" />
</div>

{#key data.pathname}
	<main in:fly={transitionIn} out:fly={transitionOut}>
		<slot />
	</main>
{/key}

<style lang="scss">
	@import "./+layout.scss";
</style>
