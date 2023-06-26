<script lang="ts">
	import "bootstrap/dist/css/bootstrap.min.css";
	import "./background.css";
	// import "bootstrap/dist/js/bootstrap.bundle";
	// import type { appWindow as aw } from "@tauri-apps/api/window";
	import { appWindow } from "@tauri-apps/api/window";
	import { invoke } from "@tauri-apps/api/tauri";
	import { page } from "$app/stores";

	const murder = async () => {
		await invoke("remote_exit");
	};

	import { fly } from "svelte/transition";
	import { cubicIn, cubicOut } from "svelte/easing";
	// import { onMount } from "svelte";

	// eslint-disable-next-line no-undef
	// let appWindow: typeof aw;

	export let data;

	// onMount(async () => {
	// 	// eslint-disable-next-line no-import-assign, @typescript-eslint/ban-ts-comment
	// 	// @ts-ignore
	// 	appWindow = await import("@tauri-apps/api/window");
	// });

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
		<a class="nav-link" href="/themes" class:active={$page.url.pathname.includes("themes")}
			>Themes</a
		>
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

{#key data.pathname}
	<main in:fly={transitionIn} out:fly={transitionOut}>
		<slot style="z-index:2;" />
	</main>
{/key}

<style lang="scss">
	@import "./+layout.scss";

	:global(.anim-cloud-click) {
		animation: clicked 150ms linear both;
	}

	@keyframes clicked {
		0% {
			filter: brightness(80%);
		}
		50% {
			filter: brightness(50%);
		}
		100% {
			filter: brightness(100%);
		}
	}
</style>
