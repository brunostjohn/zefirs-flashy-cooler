<script lang="ts">
	import type { Theme } from "../helpers/themeTools";
	import SvelteMarkdown from "svelte-markdown";
	import HeadingRendererMd from "./HeadingRendererMd.svelte";
	import { goto } from "$app/navigation";

	export let theme: Theme;

	let panel: HTMLDivElement;

	const onClickPanel = () => {
		panel.classList.remove("clicked-panel");
		panel.classList.add("clicked-panel");
		setTimeout(() => {
			panel.classList.remove("clicked-panel");
			goto(`/themes/fsName?fsName=${encodeURIComponent(theme.fs_name)}`);
		}, 150);
	};
</script>

<div class="panel-container">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div class="featured-theme" bind:this={panel} on:click={onClickPanel}>
		<img src={theme.image_src} alt="Featured theme" class="bg-img" />
		<div class="short-info" style={`color: ${theme.colour};`}>
			<h3>
				{theme.name}
			</h3>
			<h4>by {theme.author}</h4>
			<img src={theme.image_src} alt="Preview" class="preview-img" />
		</div>
		<div class="text-container" style={`color: ${theme.colour};`}>
			<div class="theme-desc">
				<SvelteMarkdown source={theme.description} renderers={{ heading: HeadingRendererMd }} />
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	@import "../styles/mixins.scss";

	:global(.clicked-panel) {
		animation: clicked-panel-animation 150ms both linear;
	}

	@keyframes clicked-panel-animation {
		0% {
			transform: scale(0.98, 0.98);
		}

		60% {
			transform: scale(0.94, 0.94);
		}

		100% {
			transform: scale(1, 1);
		}
	}

	.panel-container {
		width: 55rem;
		height: 25rem;
		position: relative;

		@include flex-center;

		&:hover {
			.featured-theme {
				transform: scale(0.98, 0.98);
			}
		}
	}

	.featured-theme {
		width: 55rem;
		height: 25rem;
		position: absolute;
		border-radius: 15px;
		overflow: hidden;

		transition: all 100ms ease-in-out;

		.bg-img {
			width: 120%;
			height: 120%;

			position: relative;
			left: -10%;
			top: -10%;

			object-fit: cover;
			z-index: -5;
			filter: blur(20px) brightness(90%);
			// background-color: var(--bs-primary);
		}

		.short-info {
			z-index: 200;
			top: 0;
			left: 3%;
			height: 100%;
			position: absolute;

			@include flex-center;
			flex-direction: column;

			h3 {
				max-width: 20rem;
				overflow: hidden;
				max-height: 2.4rem;
				text-overflow: ellipsis;
				white-space: nowrap;
			}

			h4 {
			}

			.preview-img {
				$img-size: 15rem;

				width: $img-size;
				height: $img-size;

				margin-top: 1rem;

				border-radius: 15px;
			}
		}

		.text-container {
			position: absolute;
			z-index: 200;
			bottom: 0;
			right: 3%;

			height: 100%;

			@include flex-center;

			width: 30rem;

			color: red;
		}

		.theme-desc {
			overflow-wrap: break-word;
			overflow: hidden;
			text-align: justify;
			text-overflow: ellipsis;
			display: -webkit-box;
			-webkit-line-clamp: 9;
			line-clamp: 9;
			-webkit-box-orient: vertical;
		}
	}
</style>
