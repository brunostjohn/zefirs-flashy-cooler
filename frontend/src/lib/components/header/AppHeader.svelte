<script lang="ts">
	import AppNav from "../appNav/AppNav.svelte";
	import WindowControls from "./WindowControls.svelte";
</script>

<header
	data-tauri-drag-region
	class="align-center items-center justify-evenly bg-surface-100-800-token fixed top-0 left-0 w-full h-14 flex z-10"
>
	<AppNav />
	<h3 class="h4 text-center ml-auto">Zefir's Flashy Cooler</h3>
	<WindowControls class="" />
</header>

<style>
	header {
		user-select: none;
		-webkite-user-select: none;
		-webkit-app-region: drag;
	}
</style>
