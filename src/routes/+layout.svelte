<script lang="ts">
	import "bootstrap/dist/css/bootstrap.min.css";
	import "./background.css";
	import { appWindow } from "@tauri-apps/api/window";
	import { invoke } from "@tauri-apps/api/tauri";
	import { page } from "$app/stores";
	import { topBarMessage } from "../helpers/stores";

	const murder = async () => {
		await invoke("remote_exit");
	};

	import { fade, fly } from "svelte/transition";
	import { cubicIn, cubicOut } from "svelte/easing";
	import { onMount } from "svelte";

	export let data;

	const duration = 200;
	const delay = duration + 50;
	const y = 10;

	const transitionIn = { easing: cubicOut, y, duration, delay };
	const transitionOut = { easing: cubicIn, y: -y, duration };

	document.addEventListener("contextmenu", (event) => event.preventDefault());

	let showDarkened = false;
	let topText = "";

	topBarMessage.subscribe((value) => {
		if (value !== "") {
			showDarkened = true;
			topText = value;
		} else {
			topText = "";
			showDarkened = false;
		}
	});
</script>

<div
	id="topbar"
	data-tauri-drag-region
	style={showDarkened ? "backdrop-filter: blur(60px) brightness(90%);" : undefined}
>
	{#if showDarkened}
		<img
			src="/images/android-chrome-192x192.png"
			id="titlebarlogo"
			alt="Zefir's Flashy Cooler Logo"
			transition:fade={{ duration: 150 }}
		/>
		<div class="featured-text" transition:fade={{ duration: 150 }} data-tauri-drag-region>
			<p class="featured-text-elt" data-tauri-drag-region>{topText}</p>
		</div>
	{/if}
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

<nav class="nav nav-pills nav-fill" id="controlbar">
	<a class="nav-link" href="/themes" class:active={$page.url.pathname.includes("themes")}>Themes</a>
	<a class="nav-link" href="/services" class:active={$page.url.pathname.includes("services")}
		>Services</a
	>
	<a class="nav-link" href="/renderer" class:active={$page.url.pathname === "/renderer"}
		>Now Playing</a
	>
	<a class="nav-link" href="/settings" class:active={$page.url.pathname === "/settings"}>Settings</a
	>
</nav>

<!-- <canvas class="bg-canvas" bind:this={canvas} /> -->

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
