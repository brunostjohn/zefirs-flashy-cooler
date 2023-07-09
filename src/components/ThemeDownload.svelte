<script lang="ts">
	import { fade } from "svelte/transition";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { onDestroy, onMount } from "svelte";

	let progress = 0;
	let totalSize = 0;
	let txSoFar = 0;
	let fileName = "";
	let fileCount = 0;
	let currentFile = 0;

	function formatBytes(bytes: number, decimals = 2) {
		if (!+bytes) return "0 Bytes";

		const k = 1024;
		const dm = decimals < 0 ? 0 : decimals;
		const sizes = ["bytes", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

		const i = Math.floor(Math.log(bytes) / Math.log(k));

		return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
	}

	let unlistener: UnlistenFn;

	onMount(async () => {
		unlistener = await listen("download-progress", (event) => {
			const payload = event.payload as {
				total_size: number;
				tx_so_far: number;
				file_name: string;
				file_count: number;
				current_file: number;
			};
			payload.total_size = payload.total_size * 1024;
			payload.tx_so_far = payload.tx_so_far / 2;
			progress = (payload.tx_so_far / payload.total_size) * 100;
			totalSize = payload.total_size;
			fileName = payload.file_name;
			fileCount = payload.file_count;
			currentFile = payload.current_file;
			txSoFar = payload.tx_so_far;
		});
	});

	onDestroy(async () => {
		await unlistener();
	});
</script>

<div class="everything" transition:fade data-tauri-drag-region>
	<h2>Downloading...</h2>
	<p>{fileName}: {currentFile}/{fileCount}</p>
	<div class="progress" role="progressbar" aria-label="Basic example">
		<div class="progress-bar" style={`width: ${progress}%;`} />
	</div>
	<small>{formatBytes(txSoFar)} out of {formatBytes(totalSize)}</small>
</div>

<style lang="scss">
	@import "../styles/mixins.scss";
	.everything {
		position: absolute;
		width: 100vw;
		height: 100vh;
		z-index: 100000;
		top: 0;
		left: 0;

		@include flex-center;
		flex-direction: column;

		background-color: rgba(0, 0, 0, 0.7);

		.progress {
			width: 60%;
			height: 1%;
			margin-bottom: 1rem;
		}
	}
</style>
