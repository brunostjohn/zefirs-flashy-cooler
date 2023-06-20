<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	export let src: string;
	export let themeName: string;
	export let textColour: string;
	export let fsName: string;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<article class="theme-card" id={fsName} on:click={() => invoke("apply_theme", { fsName })}>
	<img {src} alt={themeName + " Card"} class="theme-img" />
	<h5 class="theme-title" style={`color: ${textColour}`}>
		{themeName}
	</h5>
</article>

<style lang="scss">
	.theme-card {
		overflow: hidden;
		position: relative;
		transition: box-shadow 0.2s;
		flex-basis: 14rem;
		flex-grow: 1;
		min-width: 12rem;

		.theme-img {
			max-width: 100%;
			height: auto;
			margin-left: 0;
			margin-right: 0;
			display: inline-block;
			transition: all ease-in-out 0.05s;
		}

		.theme-title {
			visibility: hidden;
			display: -webkit-box;
			-webkit-box-orient: vertical;
			-webkit-line-clamp: 1;
			overflow: hidden;
			position: absolute;
			bottom: 0;
			left: 0;
			transform: translate(5px, 9px);
			transition: visibility ease-in-out 0.05s;
		}

		&:hover {
			.theme-img {
				filter: blur(5px);
				overflow: hidden;
			}

			.theme-title {
				visibility: visible;
			}
		}
	}
</style>
