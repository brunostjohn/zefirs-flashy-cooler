<script lang="ts">
	import { goto } from "$app/navigation";
	import { invoke } from "@tauri-apps/api/tauri";

	export let src: string;
	export let themeName: string;
	export let textColour: string;
	export let fsName: string;
	export let networked = false;
	const onClick = networked
		? (fsName: string) => {
				goto(`/themes/${encodeURIComponent(fsName)}`);
		  }
		: (fsName: string) =>
				invoke("apply_theme", {
					fsName,
				});
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<article class="theme-card" id={fsName} on:click={() => onClick(fsName)}>
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
		min-height: 12rem;
		aspect-ratio: 1/1;

		.theme-img {
			overflow: hidden;
			position: relative;
			width: 100%;
			height: 100%;
			height: auto;
			margin-left: 0;
			margin-right: 0;
			display: inline-block;
		}

		.theme-title {
			opacity: 0%;
			display: -webkit-box;
			-webkit-box-orient: vertical;
			-webkit-line-clamp: 1;
			overflow: hidden;
			position: absolute;
			bottom: 10px;
			left: 0;
			transform: translate(5px, 9px);
			transition: opacity ease-in-out 100ms;
		}

		&:hover {
			.theme-img {
				filter: blur(5px);
				overflow: hidden;

				object-fit: cover;

				width: 110%;
				height: 110%;
				top: -5%;
				left: -5%;
			}

			.theme-title {
				opacity: 100%;
			}
		}
	}
</style>
