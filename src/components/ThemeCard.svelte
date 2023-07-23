<script lang="ts">
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";

	// eslint-disable-next-line @typescript-eslint/ban-ts-comment
	// @ts-expect-error
	// import ColorThief from "colorthief";

	export let src: string;
	export let themeName: string;
	// export let textColour: string;
	export let fsName: string;
	// export let networked = false;
	const onClick = (fsName: string) => {
		goto(`/themes/fsName?fsName=${encodeURIComponent(fsName)}`);
	};

	let imageElt: HTMLImageElement;

	let textColourComputed: string;

	const onImageLoad = () => {
		// const ct = new ColorThief();

		// imageElt.crossOrigin = "anonymous";

		// const color = ct.getColor(imageElt);

		// const colorString = `rgb(${255 - color[0]} ${255 - color[1]} ${255 - color[2]})`;

		// textColourComputed = colorString;

		textColourComputed = "#ffffff";
	};

	onMount(() => {
		imageElt.addEventListener("load", onImageLoad);

		return () => {
			imageElt.removeEventListener("load", onImageLoad);
		};
	});
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<article class="theme-card" id={fsName} on:click={() => onClick(fsName)}>
	<img {src} alt={themeName + " Card"} class="theme-img" bind:this={imageElt} />
	<h5 class="theme-title" style={`color: ${textColourComputed}`}>
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

			transition: all ease-in-out 100ms;
		}

		.theme-title {
			opacity: 0%;
			display: -webkit-box;
			-webkit-box-orient: vertical;
			-webkit-line-clamp: 1;
			overflow: hidden;
			position: absolute;
			bottom: 6px;
			left: 1px;
			transform: translate(5px, 9px);
			transition: all ease-in-out 100ms;
		}

		&:hover {
			.theme-img {
				filter: blur(5px) brightness(0.5);
				overflow: hidden;

				object-fit: cover;

				// width: 105%;
				// height: 105%;
				// top: -2.5%;
				// left: -2.5%;

				transform: scale(1.05, 1.05);
			}

			.theme-title {
				opacity: 100%;
			}
		}
	}
</style>
